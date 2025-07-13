// this_file: crates/plugin-sdk/src/plugins/reuse_paths.rs

//! Finds <path> elements with the same d, fill, and stroke attributes and converts them to <use> elements
//!
//! This plugin optimizes SVG files by deduplicating identical path elements. It creates a single
//! path definition in <defs> and references it multiple times using <use> elements.
//!
//! Reference: SVGO's reusePaths plugin

use crate::Plugin;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use vexy_svgo_core::ast::{Document, Element, Node};

/// Configuration for the reusePaths plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReusePathsConfig {}

impl Default for ReusePathsConfig {
    fn default() -> Self {
        Self {}
    }
}

/// Plugin that deduplicates path elements by converting identical paths to <use> elements
pub struct ReusePathsPlugin {
    #[allow(dead_code)]
    config: ReusePathsConfig,
}

impl ReusePathsPlugin {
    pub fn new() -> Self {
        Self {
            #[allow(dead_code)]
            config: ReusePathsConfig::default(),
        }
    }

    pub fn with_config(config: ReusePathsConfig) -> Self {
        Self { config }
    }

    fn parse_config(params: &Value) -> Result<ReusePathsConfig> {
        if params.is_null() {
            Ok(ReusePathsConfig::default())
        } else {
            serde_json::from_value(params.clone())
                .map_err(|e| anyhow::anyhow!("Invalid plugin configuration: {}", e))
        }
    }

    /// Generates a unique key for grouping paths with identical attributes
    fn generate_path_key(element: &Element) -> Option<String> {
        let d = element.attr("d")?;
        let fill = element.attr("fill").map(|s| s.as_ref()).unwrap_or("");
        let stroke = element.attr("stroke").map(|s| s.as_ref()).unwrap_or("");

        Some(format!("{};s:{};f:{}", d, stroke, fill))
    }

    /// Collects all existing href references to avoid ID conflicts
    fn collect_hrefs(&self, element: &Element, hrefs: &mut HashSet<String>) {
        if element.name == "use" {
            // Check both href and xlink:href attributes
            for attr_name in &["href", "xlink:href"] {
                if let Some(href) = element.attr(attr_name) {
                    if href.starts_with('#') && href.len() > 1 {
                        hrefs.insert(href[1..].to_string());
                    }
                }
            }
        }

        // Recursively check children
        for child in &element.children {
            if let Node::Element(child_elem) = child {
                self.collect_hrefs(child_elem, hrefs);
            }
        }
    }

    /// Collects all path elements and groups them by their key
    fn collect_paths<'a>(
        &self,
        element: &Element<'a>,
        path_groups: &mut HashMap<String, Vec<(String, Element<'a>)>>,
        path: String,
    ) {
        if element.name == "path" && element.has_attr("d") {
            if let Some(key) = Self::generate_path_key(element) {
                path_groups
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push((path.clone(), element.clone()));
            }
        }

        // Recursively process children
        for (i, child) in element.children.iter().enumerate() {
            if let Node::Element(child_elem) = child {
                let child_path = format!("{}/{}", path, i);
                self.collect_paths(child_elem, path_groups, child_path.clone());
            }
        }
    }

    /// Finds or creates a defs element as a direct child of the svg element
    fn find_or_create_defs(&self, svg_element: &mut Element) -> Result<usize> {
        // Look for existing defs element
        for (i, child) in svg_element.children.iter().enumerate() {
            if let Node::Element(element) = child {
                if element.name == "defs" {
                    return Ok(i);
                }
            }
        }

        // Create new defs element
        let defs_element = Element::new("defs");
        svg_element.children.insert(0, Node::Element(defs_element));
        Ok(0)
    }

    /// Replaces a path element at the given path with a use element
    fn replace_path_with_use(
        &self,
        element: &mut Element,
        path: &str,
        definition_id: &str,
    ) -> bool {
        let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        if parts.is_empty() {
            return false;
        }

        let mut current = element;

        // Navigate to parent of target element
        for &part in &parts[..parts.len() - 1] {
            if let Ok(index) = part.parse::<usize>() {
                if index < current.children.len() {
                    if let Node::Element(child) = &mut current.children[index] {
                        current = child;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Replace the target element
        if let Ok(index) = parts[parts.len() - 1].parse::<usize>() {
            if index < current.children.len() {
                if let Node::Element(path_elem) = &mut current.children[index] {
                    if path_elem.name == "path" {
                        // Convert to use element
                        path_elem.name = "use".into();
                        path_elem.remove_attr("d");
                        path_elem.remove_attr("fill");
                        path_elem.remove_attr("stroke");
                        path_elem.set_attr("xlink:href", format!("#{}", definition_id));
                        path_elem.children.clear();
                        return true;
                    }
                }
            }
        }

        false
    }
}

impl Default for ReusePathsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for ReusePathsPlugin {
    fn name(&self) -> &'static str {
        "reusePaths"
    }

    fn description(&self) -> &'static str {
        "find elements with the same d, fill, and stroke, and convert them to <use>"
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply<'a>(&self, document: &mut Document<'a>) -> Result<()> {
        let mut _changed = false;

        // Collect existing href references
        let mut existing_hrefs = HashSet::new();
        self.collect_hrefs(&document.root, &mut existing_hrefs);

        // Group paths by their key (d + fill + stroke)
        let mut path_groups: HashMap<String, Vec<(String, Element)>> = HashMap::new();
        self.collect_paths(&document.root, &mut path_groups, String::new());

        // Filter groups to only those with multiple paths
        let groups_to_process: Vec<_> = path_groups
            .into_iter()
            .filter(|(_, paths)| paths.len() > 1)
            .collect();

        if groups_to_process.is_empty() {
            return Ok(());
        }

        // Ensure we have an SVG element
        if document.root.name != "svg" {
            return Ok(());
        }

        // Get or create defs element
        let defs_index = self.find_or_create_defs(&mut document.root)?;
        
        // Check if we created a new defs element at index 0
        let created_new_defs = defs_index == 0 && 
            groups_to_process.iter().any(|(_, paths)| 
                paths.iter().any(|(path, _)| path == "/0")
            );

        let mut reuse_index = 0;
        let mut has_created_uses = false;

        // Process each group of identical paths
        for (_key, mut paths) in groups_to_process {
            // If we created a new defs at index 0, adjust all path indices
            if created_new_defs {
                for (path, _elem) in &mut paths {
                    if path.starts_with("/") {
                        let parts: Vec<String> = path.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                        if let Some(first) = parts.first() {
                            if let Ok(index) = first.parse::<usize>() {
                                // Increment the index since defs was inserted at 0
                                let mut new_path = format!("/{}", index + 1);
                                if parts.len() > 1 {
                                    for part in &parts[1..] {
                                        new_path = format!("{}/{}", new_path, part);
                                    }
                                }
                                *path = new_path;
                            }
                        }
                    }
                }
            }
            if paths.len() <= 1 {
                continue;
            }

            // Create reusable path definition from the first path
            let (_, first_path) = &paths[0];
            let mut reusable_path = first_path.clone();

            // Keep only d, fill, and stroke attributes for the definition
            let d_attr = reusable_path
                .attr("d")
                .map(|s| s.as_ref())
                .unwrap_or("")
                .to_string();
            let fill_attr = reusable_path.attr("fill").map(|s| s.to_string());
            let stroke_attr = reusable_path.attr("stroke").map(|s| s.to_string());

            // Clear all attributes and set only the needed ones
            reusable_path.attributes.clear();
            reusable_path.set_attr("d", d_attr);
            if let Some(fill) = fill_attr {
                reusable_path.set_attr("fill", fill);
            }
            if let Some(stroke) = stroke_attr {
                reusable_path.set_attr("stroke", stroke);
            }

            // Handle ID assignment for the definition
            let original_id = first_path.attr("id");
            let definition_id = if let Some(id) = original_id {
                if existing_hrefs.contains(&id.to_string()) {
                    // ID is already referenced, create new one
                    let new_id = format!("reuse-{}", reuse_index);
                    reuse_index += 1;
                    new_id
                } else {
                    // Use existing ID
                    id.to_string()
                }
            } else {
                // No ID, create new one
                let new_id = format!("reuse-{}", reuse_index);
                reuse_index += 1;
                new_id
            };

            reusable_path.set_attr("id", &definition_id);
            reusable_path.children.clear();

            // Add the reusable path to defs
            if let Node::Element(defs) = &mut document.root.children[defs_index] {
                defs.children.push(Node::Element(reusable_path));
            }

            // Convert all paths in this group to use elements
            for (path, _) in &paths {
                if self.replace_path_with_use(&mut document.root, path, &definition_id) {
                    has_created_uses = true;
                    _changed = true;
                }
            }
        }

        // Add xmlns:xlink namespace if we created any use elements
        if has_created_uses && !document.root.has_attr("xmlns:xlink") {
            document
                .root
                .set_attr("xmlns:xlink", "http://www.w3.org/1999/xlink");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vexy_svgo_core::ast::{Document, Element, Node};

    #[test]
    fn test_plugin_info() {
        let plugin = ReusePathsPlugin::new();
        assert_eq!(plugin.name(), "reusePaths");
        assert_eq!(
            plugin.description(),
            "find elements with the same d, fill, and stroke, and convert them to <use>"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = ReusePathsPlugin::new();

        // Test null params
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Test empty object params
        assert!(plugin.validate_params(&serde_json::json!({})).is_ok());

        // Test invalid params
        assert!(plugin
            .validate_params(&serde_json::json!({
                "invalidParam": true
            }))
            .is_err());
    }

    #[test]
    fn test_reuse_identical_paths() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <path d="M10,10 L20,20" fill="red" stroke="blue"/>
            <path d="M10,10 L20,20" fill="red" stroke="blue"/>
        </svg>"#;

        let mut document = vexy_svgo_core::parse_svg(svg).unwrap();
        let plugin = ReusePathsPlugin::new();

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Check that defs was created
        let defs_exists = document.root.children.iter().any(|child| {
            if let Node::Element(elem) = child {
                elem.name == "defs"
            } else {
                false
            }
        });
        assert!(defs_exists);

        // Check that paths were converted to use elements
        let use_count = count_elements(&document.root, "use");
        assert_eq!(use_count, 2);

        // Check that xmlns:xlink was added
        assert!(document.root.has_attr("xmlns:xlink"));
    }

    #[test]
    fn test_different_paths_not_reused() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <path d="M10,10 L20,20" fill="red" stroke="blue"/>
            <path d="M30,30 L40,40" fill="red" stroke="blue"/>
        </svg>"#;

        let mut document = vexy_svgo_core::parse_svg(svg).unwrap();
        let plugin = ReusePathsPlugin::new();

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Check that no use elements were created
        let use_count = count_elements(&document.root, "use");
        assert_eq!(use_count, 0);

        // Check that paths remain unchanged
        let path_count = count_elements(&document.root, "path");
        assert_eq!(path_count, 2);
    }

    #[test]
    fn test_path_key_generation() {
        let mut element = Element::new("path");
        element.set_attr("d", "M10,10 L20,20");
        element.set_attr("fill", "red");
        element.set_attr("stroke", "blue");

        let key = ReusePathsPlugin::generate_path_key(&element);
        assert_eq!(key, Some("M10,10 L20,20;s:blue;f:red".to_string()));
    }

    #[test]
    fn test_path_without_d_attribute() {
        let mut element = Element::new("path");
        element.set_attr("fill", "red");

        let key = ReusePathsPlugin::generate_path_key(&element);
        assert_eq!(key, None);
    }

    #[test]
    fn test_existing_defs_reused() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg">
            <defs>
                <circle id="myCircle" r="5"/>
            </defs>
            <path d="M10,10 L20,20" fill="red"/>
            <path d="M10,10 L20,20" fill="red"/>
        </svg>"#;

        let mut document = vexy_svgo_core::parse_svg(svg).unwrap();
        let plugin = ReusePathsPlugin::new();

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Check that existing defs was reused (should still have only one defs)
        let defs_count = count_elements(&document.root, "defs");
        assert_eq!(defs_count, 1);

        // Check that the original circle is still there
        let circle_exists = document.root.children.iter().any(|child| {
            if let Node::Element(elem) = child {
                if elem.name == "defs" {
                    elem.children.iter().any(|defs_child| {
                        if let Node::Element(defs_elem) = defs_child {
                            defs_elem.name == "circle"
                                && defs_elem.attr("id").map(|s| s.as_str()) == Some("myCircle")
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            } else {
                false
            }
        });
        assert!(circle_exists);
    }

    fn count_elements(element: &Element, name: &str) -> usize {
        let mut count = 0;
        if element.name == name {
            count += 1;
        }
        for child in &element.children {
            if let Node::Element(child_elem) = child {
                count += count_elements(child_elem, name);
            }
        }
        count
    }
}
