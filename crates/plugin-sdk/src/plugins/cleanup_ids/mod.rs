// this_file: crates/plugin-sdk/src/plugins/cleanup_ids.rs

//! Cleanup IDs plugin implementation
//!
//! This plugin removes unused IDs and minifies used IDs to save space.
//! It// Add elements with IDs to preservestatic str {
        "cleanupIds"
    }

    fn description(&self) -> &'static str {
        PROTECTED_5_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        if let Some(obj) = params.as_object() {
            // Validate parameters
            for (key, value) in obj {
                match key.as_str() {
                    PROTECTED_6_ | PROTECTED_7_ | PROTECTED_8_ => {
                        if !value.is_boolean() {
                            return Err(anyhow!(PROTECTED_9_, key));
                        }
                    }
                    PROTECTED_10_ | PROTECTED_11_ => {
                        if !value.is_array() {
                            return Err(anyhow!(PROTECTED_12_, key));
                        }
                        if let Some(arr) = value.as_array() {
                            for item in arr {
                                if !item.is_string() {
                                    return Err(anyhow!(PROTECTED_13_, key));
                                }
                            }
                        }
                    }
                    _ => return Err(anyhow!(PROTECTED_14_, key)),
                }
            }
        }
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        // First pass: collect all IDs and references
        let mut collector = CleanupIdsVisitor::new(self.config.clone());
        vexy_svgo_core::visitor::walk_document(&mut collector, document)?;
        
        // Check if we should skip processing
        if !collector.config.force && (collector.has_scripts || collector.has_styles) {
            return Ok(());
        }

        // Determine which IDs are used
        let used_ids: HashSet<String> = collector.references_by_id.keys().cloned().collect();

        // Generate ID mappings for minification
        let mut id_mappings = HashMap::new();
        if collector.config.minify {
            let mut id_generator = IdGenerator::new();
            let mut new_id_set = HashSet::new();

            // Process referenced IDs
            for id in &used_ids {
                if !collector.is_id_preserved(id) && collector.node_by_id.contains_key(id) {
                    // Generate unique minified ID
                    let mut new_id = id_generator.next();
                    while collector.is_id_preserved(&new_id)
                        || collector.node_by_id.contains_key(&new_id)
                        || new_id_set.contains(&new_id)
                    {
                        new_id = id_generator.next();
                    }
                    new_id_set.insert(new_id.clone());
                    id_mappings.insert(id.clone(), new_id);
                }
            }
        }

        // Second pass: apply changes
        let mut applier = IdApplierVisitor {
            config: &collector.config,
            node_by_id: &collector.node_by_id,
            used_ids: &used_ids,
            id_mappings: &id_mappings,
            current_path: Vec::new(),
        };
        vexy_svgo_core::visitor::walk_document(&mut applier, document)?;

        Ok(())
    }
}

/// Visitor implementation that cleans up IDs
struct CleanupIdsVisitor {
    config: CleanupIdsConfig,
    has_scripts: bool,
    has_styles: bool,
    node_by_id: HashMap<String, ElementInfo>,
    references_by_id: HashMap<String, Vec<Reference>>,
    current_path: Vec<usize>,
}

#[derive(Debug, Clone)]
struct ElementInfo {
    path: Vec<usize>,
    element_name: String,
}

#[derive(Debug, Clone)]
struct Reference {
    path: Vec<usize>,
    attr_name: String,
}

impl CleanupIdsVisitor {
    fn new(config: CleanupIdsConfig) -> Self {
        Self {
            config,
            has_scripts: false,
            has_styles: false,
            node_by_id: HashMap::new(),
            references_by_id: HashMap::new(),
            current_path: Vec::new(),
        }
    }

    fn check_for_scripts(&mut self, element: &Element) {
        // Check for script elements
        if element.name == PROTECTED_15_
            && element
                .children
                .iter()
                .any(|n| matches!(n, Node::Text(_) | Node::CData(_)))
        {
            self.has_scripts = true;
            return;
        }

        // Check for javascript: links
        if element.name == PROTECTED_16_ {
            if let Some(href) = element.attributes.get(PROTECTED_17_) {
                if href.trim_start().starts_with(PROTECTED_18_) {
                    self.has_scripts = true;
                    return;
                }
            }
        }

        // Check for event attributes
        for (attr_name, _) in &element.attributes {
            if attr_name.starts_with(PROTECTED_19_) {
                self.has_scripts = true;
                return;
            }
        }
    }

    fn check_for_styles(&mut self, element: &Element) {
        if element.name == PROTECTED_20_
            && element
                .children
                .iter()
                .any(|n| matches!(n, Node::Text(_) | Node::CData(_)))
        {
            self.has_styles = true;
        }
    }

    fn is_id_preserved(&self, id: &str) -> bool {
        self.config.preserve.contains(&id.to_string())
            || self
                .config
                .preserve_prefixes
                .iter()
                .any(|prefix| id.starts_with(prefix))
    }

    fn collect_references(&mut self, element: &Element) {
        for (attr_name, attr_value) in &element.attributes {
            let ids = find_references(attr_name, attr_value);
            for id in ids {
                self.references_by_id
                    .entry(id)
                    .or_insert_with(Vec::new)
                    .push(Reference {
                        path: self.current_path.clone(),
                        attr_name: attr_name.clone(),
                    });
            }
        }
    }
}

impl Visitor<'_> for CleanupIdsVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Update path
        let _child_index = self.current_path.len();
        self.current_path.push(0);

        // Check for scripts and styles
        if !self.has_scripts {
            self.check_for_scripts(element);
        }
        if !self.has_styles {
            self.check_for_styles(element);
        }

        // Collect ID if present
        if let Some(id) = element.attributes.get(PROTECTED_21_) {
            self.node_by_id.insert(
                id.clone(),
                ElementInfo {
                    path: self.current_path.clone(),
                    element_name: element.name.to_string(),
                },
            );
        }

        // Collect references
        self.collect_references(element);

        Ok(())
    }

    fn visit_element_exit(&mut self, _element: &mut Element<'_>) -> Result<()> {
        // Update path
        self.current_path.pop();
        if !self.current_path.is_empty() {
            let last_idx = self.current_path.len() - 1;
            self.current_path[last_idx] += 1;
        }
        Ok(())
    }
}

/// Separate visitor for applying ID changes
struct IdApplierVisitor<'a> {
    config: &'a CleanupIdsConfig,
    node_by_id: &'a HashMap<String, ElementInfo>,
    used_ids: &'a HashSet<String>,
    id_mappings: &'a HashMap<String, String>,
    current_path: Vec<usize>,
}

impl<'a> IdApplierVisitor<'a> {
    fn is_id_preserved(&self, id: &str) -> bool {
        self.config.preserve.contains(&id.to_string())
            || self
                .config
                .preserve_prefixes
                .iter()
                .any(|prefix| id.starts_with(prefix))
    }
}

impl Visitor<'_> for IdApplierVisitor<'_> {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Update path
        let _child_index = self.current_path.len();
        self.current_path.push(0);

        // Process ID attribute
        if let Some(id) = element.attributes.get("id").cloned() {
            if let Some(new_id) = self.id_mappings.get(&id) {
                // Update to minified ID
                element.attributes.insert("id".to_string(), new_id.clone());
            } else if self.config.remove
                && !self.used_ids.contains(&id)
                && !self.is_id_preserved(&id)
            {
                // Remove unused ID
                element.attributes.shift_remove("id");
            }
        }

        // Update references in attributes
        let mut updates = Vec::new();
        for (attr_name, attr_value) in &element.attributes {
            if REFERENCES_PROPS.contains(&attr_name.as_str())
                || attr_name == "style"
                || attr_name == "href"
                || attr_name == "xlink:href"
                || attr_name == "begin"
            {
                let new_value = update_reference_value(attr_value, self.id_mappings);
                if new_value != *attr_value {
                    updates.push((attr_name.clone(), new_value));
                }
            }
        }

        // Apply updates
        for (attr_name, new_value) in updates {
            element.attributes.insert(attr_name, new_value);
        }

        Ok(())
    }

    fn visit_element_exit(&mut self, _element: &mut Element<'_>) -> Result<()> {
        // Update path
        self.current_path.pop();
        if !self.current_path.is_empty() {
            let last_idx = self.current_path.len() - 1;
            self.current_path[last_idx] += 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::borrow::Cow;
    use vexy_svgo_core::ast::{Document, Element, Node};

    fn create_element(name: &'static str) -> Element<'static> {
        let mut element = Element::new(name);
        element.name = Cow::Borrowed(name);
        element
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = CleanupIdsPlugin::new();
        assert_eq!(plugin.name(), PROTECTED_29_);
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = CleanupIdsPlugin::new();

        // Valid parameters
        assert!(plugin.validate_params(&json!({})).is_ok());
        assert!(plugin
            .validate_params(&json!({
                PROTECTED_30_: true,
                PROTECTED_31_: false,
                PROTECTED_32_: [PROTECTED_33_, PROTECTED_34_],
                PROTECTED_35_: [PROTECTED_36_],
                PROTECTED_37_: true
            }))
            .is_ok());

        // Invalid parameters
        assert!(plugin
            .validate_params(&json!({PROTECTED_38_: PROTECTED_39_}))
            .is_err());
        assert!(plugin
            .validate_params(&json!({PROTECTED_40_: PROTECTED_41_}))
            .is_err());
        assert!(plugin.validate_params(&json!({PROTECTED_42_: [123]})).is_err());
        assert!(plugin
            .validate_params(&json!({PROTECTED_43_: true}))
            .is_err());
    }

    #[test]
    fn test_find_references() {
        // Test href references
        assert_eq!(find_references(PROTECTED_44_, PROTECTED_45_), vec![PROTECTED_46_]);
        assert_eq!(find_references(PROTECTED_47_, PROTECTED_48_), vec![PROTECTED_49_]);

        // Test URL references
        assert_eq!(find_references(PROTECTED_50_, PROTECTED_51_), vec![PROTECTED_52_]);
        assert_eq!(find_references(PROTECTED_53_, PROTECTED_54_), vec![PROTECTED_55_]);
        assert_eq!(find_references(PROTECTED_56_, PROTECTED_57_), vec![PROTECTED_58_]);

        // Test style attribute
        assert_eq!(
            find_references(PROTECTED_59_, PROTECTED_60_),
            vec![PROTECTED_61_, PROTECTED_62_]
        );

        // Test begin attribute
        assert_eq!(find_references(PROTECTED_63_, PROTECTED_64_), vec![PROTECTED_65_]);
    }

    #[test]
    fn test_id_generator() {
        let mut gen = IdGenerator::new();
        assert_eq!(gen.next(), PROTECTED_66_);
        assert_eq!(gen.next(), PROTECTED_67_);
        assert_eq!(gen.next(), PROTECTED_68_);

        // Test rollover - generate IDs until we reach 'Z', then test wraparound
        // Skip to near the end (we already have 'a', 'b', 'c', so skip 47 more to get to 'Y')
        for _ in 0..47 {
            gen.next();
        }
        assert_eq!(gen.next(), "Y");
        assert_eq!(gen.next(), "Z");
        assert_eq!(gen.next(), "aa");
        assert_eq!(gen.next(), "ab");
    }

    #[test]
    fn test_remove_unused_ids() {
        let plugin = CleanupIdsPlugin::new();
        let mut doc = Document::new();

        // Add elements with IDs
        let mut rect1 = create_element("rect");
        rect1
            .attributes
            .insert("id".to_string(), "used".to_string());

        let mut rect2 = create_element("rect");
        rect2
            .attributes
            .insert("id".to_string(), "unused".to_string());

        let mut use_elem = create_element("use");
        use_elem
            .attributes
            .insert("href".to_string(), "#used".to_string());

        doc.root.children.push(Node::Element(rect1));
        doc.root.children.push(Node::Element(rect2));
        doc.root.children.push(Node::Element(use_elem));

        plugin.apply(&mut doc).unwrap();

        // Check that used ID is kept and unused is removed
        if let Some(Node::Element(rect1)) = doc.root.children.get(0) {
            assert!(rect1.attributes.contains_key("id"));
        }
        if let Some(Node::Element(rect2)) = doc.root.children.get(1) {
            assert!(!rect2.attributes.contains_key("id"));
        }
    }

    #[test]
    fn test_minify_ids() {
        let config = CleanupIdsConfig {
            remove: false,
            minify: true,
            ..Default::default()
        };
        let plugin = CleanupIdsPlugin::with_config(config);
        let mut doc = Document::new();

        // Add element with long ID
        let mut rect = create_element("rect");
        rect.attributes
            .insert("id".to_string(), "very_long_identifier".to_string());

        let mut use_elem = create_element("use");
        use_elem
            .attributes
            .insert("href".to_string(), "#very_long_identifier".to_string());

        doc.root.children.push(Node::Element(rect));
        doc.root.children.push(Node::Element(use_elem));

        plugin.apply(&mut doc).unwrap();

        // Check that ID was minified
        if let Some(Node::Element(rect)) = doc.root.children.get(0) {
            let id = rect.attributes.get("id").unwrap();
            assert!(id.len() < "very_long_identifier".len());
            assert_eq!(id, "a"); // First generated ID should be PROTECTED_93_
        }

        // Check that reference was updated
        if let Some(Node::Element(use_elem)) = doc.root.children.get(1) {
            assert_eq!(use_elem.attributes.get("href"), Some(&"#a".to_string()));
        }
    }

    #[test]
    fn test_preserve_ids() {
        let config = CleanupIdsConfig {
            remove: true,
            minify: true,
            preserve: vec!["keep_this".to_string()],
            preserve_prefixes: vec!["pres_".to_string()],
            ..Default::default()
        };
        let plugin = CleanupIdsPlugin::with_config(config);
        let mut doc = Document::new();

        // Add elements with IDs to preserve
        let mut rect1 = create_element("rect");
        rect1
            .attributes
            .insert("id".to_string(), "keep_this".to_string());

        let mut rect2 = create_element("rect");
        rect2
            .attributes
            .insert("id".to_string(), "pres_something".to_string());

        let mut rect3 = create_element("rect");
        rect3
            .attributes
            .insert("id".to_string(), "remove_this".to_string());

        doc.root.children.push(Node::Element(rect1));
        doc.root.children.push(Node::Element(rect2));
        doc.root.children.push(Node::Element(rect3));

        plugin.apply(&mut doc).unwrap();

        // Check preserved IDs
        if let Some(Node::Element(rect1)) = doc.root.children.get(0) {
            assert_eq!(rect1.attributes.get("id"), Some(&"keep_this".to_string()));
        }
        if let Some(Node::Element(rect2)) = doc.root.children.get(1) {
            assert_eq!(
                rect2.attributes.get("id"),
                Some(&"pres_something".to_string())
            );
        }
        if let Some(Node::Element(rect3)) = doc.root.children.get(2) {
            assert!(!rect3.attributes.contains_key("id"));
        }
    }

    #[test]
    fn test_skip_with_scripts() {
        let plugin = CleanupIdsPlugin::new();
        let mut doc = Document::new();

        // Add script element
        let mut script = create_element("script");
        script
            .children
            .push(Node::Text("console.log('test');".to_string()));

        // Add element with ID
        let mut rect = create_element("rect");
        rect.attributes
            .insert("id".to_string(), "should_not_be_removed".to_string());

        doc.root.children.push(Node::Element(script));
        doc.root.children.push(Node::Element(rect));

        plugin.apply(&mut doc).unwrap();

        // ID should still be there because of script
        if let Some(Node::Element(rect)) = doc.root.children.get(1) {
            assert_eq!(
                rect.attributes.get("id"),
                Some(&"should_not_be_removed".to_string())
            );
        }
    }

    #[test]
    fn test_force_with_scripts() {
        let config = CleanupIdsConfig {
            remove: true,
            force: true,
            ..Default::default()
        };
        let plugin = CleanupIdsPlugin::with_config(config);
        let mut doc = Document::new();

        // Add script element
        let mut script = create_element("script");
        script
            .children
            .push(Node::Text("console.log('test');".to_string()));

        // Add element with unused ID
        let mut rect = create_element("rect");
        rect.attributes
            .insert("id".to_string(), "should_be_removed".to_string());

        doc.root.children.push(Node::Element(script));
        doc.root.children.push(Node::Element(rect));

        plugin.apply(&mut doc).unwrap();

        // ID should be removed because force is true
        if let Some(Node::Element(rect)) = doc.root.children.get(1) {
            assert!(!rect.attributes.contains_key("id"));
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = CleanupIdsPlugin::parse_config(&json!({
            "remove": false,
            "minify": true,
            "preserve": ["id1", "id2"],
            "preservePrefixes": ["prefix_"],
            "force": false
        }))
        .unwrap();

        assert_eq!(config.remove, false);
        assert_eq!(config.minify, true);
        assert_eq!(config.preserve, vec!["id1", "id2"]);
        assert_eq!(config.preserve_prefixes, vec!["prefix_"]);
        assert_eq!(config.force, false);
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(CleanupIdsPlugin, "cleanupIds");
        doc.root.children.push(Node::Element(rect2));
        doc.root.children.push(Node::Element(rect3));

        plugin.apply(&mut doc).unwrap();

        // Check preserved IDs
        if let Some(Node::Element(rect1)) = doc.root.children.get(0) {
            assert_eq!(rect1.attributes.get("id"), Some(&"keep_this".to_string()));
        }
        if let Some(Node::Element(rect2)) = doc.root.children.get(1) {
            assert_eq!(
                rect2.attributes.get("id"),
                Some(&"pres_something".to_string())
            );
        }
        if let Some(Node::Element(rect3)) = doc.root.children.get(2) {
            assert!(!rect3.attributes.contains_key("id"));
        }
    }

    #[test]
    fn test_skip_with_scripts() {
        let plugin = CleanupIdsPlugin::new();
        let mut doc = Document::new();

        // Add script element
        let mut script = create_element("script");
        script
            .children
            .push(Node::Text("console.log('test');".to_string()));

        // Add element with ID
        let mut rect = create_element("rect");
        rect.attributes
            .insert("id".to_string(), "should_not_be_removed".to_string());

        doc.root.children.push(Node::Element(script));
        doc.root.children.push(Node::Element(rect));

        plugin.apply(&mut doc).unwrap();

        // ID should still be there because of script
        if let Some(Node::Element(rect)) = doc.root.children.get(1) {
            assert_eq!(
                rect.attributes.get("id"),
                Some(&"should_not_be_removed".to_string())
            );
        }
    }

    #[test]
    fn test_force_with_scripts() {
        let config = CleanupIdsConfig {
            remove: true,
            force: true,
            ..Default::default()
        };
        let plugin = CleanupIdsPlugin::with_config(config);
        let mut doc = Document::new();

        // Add script element
        let mut script = create_element("script");
        script
            .children
            .push(Node::Text("console.log('test');".to_string()));

        // Add element with unused ID
        let mut rect = create_element("rect");
        rect.attributes
            .insert("id".to_string(), "should_be_removed".to_string());

        doc.root.children.push(Node::Element(script));
        doc.root.children.push(Node::Element(rect));

        plugin.apply(&mut doc).unwrap();

        // ID should be removed because force is true
        if let Some(Node::Element(rect)) = doc.root.children.get(1) {
            assert!(!rect.attributes.contains_key("id"));
        }
    }

    #[test]
    fn test_config_parsing() {
        let config = CleanupIdsPlugin::parse_config(&json!({
            "remove": false,
            "minify": true,
            "preserve": ["id1", "id2"],
            "preservePrefixes": ["prefix_"],
            "force": false
        }))
        .unwrap();

        assert_eq!(config.remove, false);
        assert_eq!(config.minify, true);
        assert_eq!(config.preserve, vec!["id1", "id2"]);
        assert_eq!(config.preserve_prefixes, vec!["prefix_"]);
        assert_eq!(config.force, false);
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(CleanupIdsPlugin, "cleanupIds");
