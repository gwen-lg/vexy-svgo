// this_file: crates/plugin-sdk/src/plugins/remove_view_box.rs

//! Remove viewBox plugin implementation
//!
//! This plugin removes viewBox attribute when it coincides with width/height box.
//! It follows the same logic as SVGOPROTECTED_113_static HashSet<&PROTECTED_114_static str>> =
            std::sync::OnceLock::new();
        VIEWBOX_ELEMENTS.get_or_init(|| ["pattern", "svg", "symbol"].into_iter().collect())
    }

    /// Check if viewBox can be removed
    fn can_remove_viewbox(element: &Element, is_nested_svg: bool) -> bool {
        // DonPROTECTED_115_ PROTECTED_116_,PROTECTED_117_pxPROTECTED_118_static str {
        "removeViewBox"
    }

    fn description(&self) -> &'static str {
        PROTECTED_12_
    }

    fn validate_params(&self, params: &serde_json::Value) -> anyhow::Result<()> {
        if let Some(obj) = params.as_object() {
            if !obj.is_empty() {
                return Err(anyhow::anyhow!(
                    PROTECTED_13_
                ));
            }
        }
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> anyhow::Result<()> {
        let mut visitor = ViewBoxRemovalVisitor::new();
        vexy_svgo_core::visitor::walk_document(&mut visitor, document)?;
        Ok(())
    }
}

/// Visitor implementation that removes viewBox attributes
struct ViewBoxRemovalVisitor {
    element_stack: Vec<String>, // Track element hierarchy for nested SVG detection
}

impl ViewBoxRemovalVisitor {
    fn new() -> Self {
        Self {
            element_stack: Vec::new(),
        }
    }

    /// Check if current element is a nested SVG
    fn is_nested_svg(&self, element_name: &str) -> bool {
        element_name == PROTECTED_14_ && self.element_stack.len() > 0
    }
}

impl Visitor<'_> for ViewBoxRemovalVisitor {
    fn visit_element_enter(&mut self, element: &mut Element<'_>) -> Result<()> {
        // Check if this element can have viewBox removed (before pushing to stack)
        if RemoveViewBoxPlugin::viewbox_elements().contains(element.name.as_ref()) {
            let is_nested_svg = self.is_nested_svg(&element.name);

            if RemoveViewBoxPlugin::can_remove_viewbox(element, is_nested_svg) {
                element.attributes.shift_remove(PROTECTED_15_);
            }
        }

        // Push current element to stack
        self.element_stack.push(element.name.to_string());

        Ok(())
    }

    fn visit_element_exit(&mut self, _element: &mut Element<'_>) -> Result<()> {
        // Pop current element from stack
        self.element_stack.pop();
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

    fn create_element_with_attrs(name: &'static str, attrs: &[(&str, &str)]) -> Element<'static> {
        let mut element = create_element(name);
        for (key, value) in attrs {
            element
                .attributes
                .insert(key.to_string(), value.to_string());
        }
        element
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = RemoveViewBoxPlugin::new();
        assert_eq!(plugin.name(), "removeViewBox");
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveViewBoxPlugin::new();

        // Valid parameters (empty object)
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Invalid parameters (non-empty object)
        assert!(plugin.validate_params(&json!({"param": true})).is_err());
    }

    #[test]
    fn test_viewbox_elements() {
        let elements = RemoveViewBoxPlugin::viewbox_elements();
        assert!(elements.contains("svg"));
        assert!(elements.contains("pattern"));
        assert!(elements.contains("symbol"));
        assert!(!elements.contains("rect"));
    }

    #[test]
    fn test_can_remove_viewbox() {
        // Case 1: Can remove - matching dimensions
        let element = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "0 0 100 50"),
                ("width", "100"),
                ("height", "50"),
            ],
        );
        assert!(RemoveViewBoxPlugin::can_remove_viewbox(&element, false));

        // Case 2: Can remove - with px suffix
        let element = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "0 0 100.5 0.5"),
                ("width", "100.5px"),
                ("height", "0.5px"),
            ],
        );
        assert!(RemoveViewBoxPlugin::can_remove_viewbox(&element, false));

        // Case 3: Cannot remove - nested SVG
        let element = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "0 0 100 50"),
                ("width", "100"),
                ("height", "50"),
            ],
        );
        assert!(!RemoveViewBoxPlugin::can_remove_viewbox(&element, true));

        // Case 4: Cannot remove - non-zero origin
        let element = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "10 10 100 50"),
                ("width", "100"),
                ("height", "50"),
            ],
        );
        assert!(!RemoveViewBoxPlugin::can_remove_viewbox(&element, false));

        // Case 5: Cannot remove - mismatched dimensions
        let element = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "0 0 100 50"),
                ("width", "200"),
                ("height", "50"),
            ],
        );
        assert!(!RemoveViewBoxPlugin::can_remove_viewbox(&element, false));

        // Case 6: Cannot remove - missing attributes
        let element =
            create_element_with_attrs("svg", &[("viewBox", "0 0 100 50"), ("width", "100")]);
        assert!(!RemoveViewBoxPlugin::can_remove_viewbox(&element, false));
    }

    #[test]
    fn test_visitor_nesting_detection() {
        let mut visitor = ViewBoxRemovalVisitor::new();

        // Root level - not nested (empty stack)
        assert!(!visitor.is_nested_svg("svg"));

        // Second level - nested (has parent element)
        visitor.element_stack.push("g".to_string());
        assert!(visitor.is_nested_svg("svg"));

        // Non-SVG element - not nested
        visitor.element_stack.clear();
        visitor.element_stack.push("g".to_string());
        assert!(!visitor.is_nested_svg("rect"));
    }

    #[test]
    fn test_plugin_apply() {
        let plugin = RemoveViewBoxPlugin::new();
        let mut doc = Document::new();

        // Set attributes on the root SVG element
        doc.root
            .attributes
            .insert("viewBox".to_string(), "0 0 100 50".to_string());
        doc.root
            .attributes
            .insert("width".to_string(), "100".to_string());
        doc.root
            .attributes
            .insert("height".to_string(), "50".to_string());

        plugin.apply(&mut doc).unwrap();

        // Check that viewBox was removed
        assert!(!doc.root.attributes.contains_key("viewBox"));
        assert!(doc.root.attributes.contains_key("width"));
        assert!(doc.root.attributes.contains_key("height"));
    }

    #[test]
    fn test_plugin_apply_preservation() {
        let plugin = RemoveViewBoxPlugin::new();
        let mut doc = Document::new();

        // Set attributes on the root SVG element with non-removable viewBox
        doc.root
            .attributes
            .insert("viewBox".to_string(), "10 10 100 50".to_string());
        doc.root
            .attributes
            .insert("width".to_string(), "100".to_string());
        doc.root
            .attributes
            .insert("height".to_string(), "50".to_string());

        plugin.apply(&mut doc).unwrap();

        // Check that viewBox was preserved
        assert!(doc.root.attributes.contains_key("viewBox"));
        assert_eq!(
            doc.root.attributes.get("viewBox"),
            Some(&"10 10 100 50".to_string())
        );
    }

    #[test]
    fn test_plugin_apply_nested_svg() {
        let plugin = RemoveViewBoxPlugin::new();
        let mut doc = Document::new();

        // Set attributes on the root SVG element
        doc.root
            .attributes
            .insert("viewBox".to_string(), "0 0 200 100".to_string());
        doc.root
            .attributes
            .insert("width".to_string(), "200".to_string());
        doc.root
            .attributes
            .insert("height".to_string(), "100".to_string());

        // Create nested SVG element
        let nested_svg = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "0 0 100 50"),
                ("width", "100"),
                ("height", "50"),
            ],
        );

        doc.root.children.push(Node::Element(nested_svg));

        plugin.apply(&mut doc).unwrap();

        // Check that outer SVG viewBox was removed but nested SVG viewBox was preserved
        assert!(!doc.root.attributes.contains_key("viewBox"));

        if let Some(Node::Element(nested_svg)) = doc.root.children.first() {
            assert!(nested_svg.attributes.contains_key("viewBox"));
            assert_eq!(
                nested_svg.attributes.get("viewBox"),
                Some(&"0 0 100 50".to_string())
            );
        }
    }

    #[test]
    fn test_plugin_apply_pattern_element() {
        let plugin = RemoveViewBoxPlugin::new();
        let mut doc = Document::new();

        // Create pattern element with removable viewBox
        let pattern_element = create_element_with_attrs(
            "pattern",
            &[("viewBox", "0 0 10 10"), ("width", "10"), ("height", "10")],
        );

        doc.root.children.push(Node::Element(pattern_element));

        plugin.apply(&mut doc).unwrap();

        // Check that viewBox was removed
        if let Some(Node::Element(pattern)) = doc.root.children.first() {
            assert!(!pattern.attributes.contains_key("viewBox"));
            assert!(pattern.attributes.contains_key("width"));
            assert!(pattern.attributes.contains_key("height"));
        }
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveViewBoxPlugin, "removeViewBox");
        doc.root
            .attributes
            .insert("viewBox".to_string(), "0 0 200 100".to_string());
        doc.root
            .attributes
            .insert("width".to_string(), "200".to_string());
        doc.root
            .attributes
            .insert("height".to_string(), "100".to_string());

        // Create nested SVG element
        let nested_svg = create_element_with_attrs(
            "svg",
            &[
                ("viewBox", "0 0 100 50"),
                ("width", "100"),
                ("height", "50"),
            ],
        );

        doc.root.children.push(Node::Element(nested_svg));

        plugin.apply(&mut doc).unwrap();

        // Check that outer SVG viewBox was removed but nested SVG viewBox was preserved
        assert!(!doc.root.attributes.contains_key("viewBox"));

        if let Some(Node::Element(nested_svg)) = doc.root.children.first() {
            assert!(nested_svg.attributes.contains_key("viewBox"));
            assert_eq!(
                nested_svg.attributes.get("viewBox"),
                Some(&"0 0 100 50".to_string())
            );
        }
    }

    #[test]
    fn test_plugin_apply_pattern_element() {
        let plugin = RemoveViewBoxPlugin::new();
        let mut doc = Document::new();

        // Create pattern element with removable viewBox
        let pattern_element = create_element_with_attrs(
            "pattern",
            &[("viewBox", "0 0 10 10"), ("width", "10"), ("height", "10")],
        );

        doc.root.children.push(Node::Element(pattern_element));

        plugin.apply(&mut doc).unwrap();

        // Check that viewBox was removed
        if let Some(Node::Element(pattern)) = doc.root.children.first() {
            assert!(!pattern.attributes.contains_key("viewBox"));
            assert!(pattern.attributes.contains_key("width"));
            assert!(pattern.attributes.contains_key("height"));
        }
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveViewBoxPlugin, "removeViewBox");
