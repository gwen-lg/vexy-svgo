// this_file: crates/plugin-sdk/src/plugins/move_elems_attrs_to_group.rs

//! Moves common attributes from child elements to their parent group
//!
//! This plugin optimizes SVG by moving attributes that all child elements
//! share to their parent group element, reducing redundancy.
//!
//! Reference: SVGOPROTECTED_103_static str> {
        [
            "fill",
            "stroke",
            "stroke-width",
            "stroke-linecap",
            "stroke-linejoin",
            "stroke-miterlimit",
            "stroke-dasharray",
            "stroke-dashoffset",
            "stroke-opacity",
            "fill-opacity",
            "opacity",
            "color",
            "font-family",
            "font-size",
            "font-style",
            "font-variant",
            "font-weight",
            "text-anchor",
            "text-decoration",
            "letter-spacing",
            "word-spacing",
        ]
        .iter()
        .copied()
        .collect()
    }

    /// Find common attributes among all child elements
    fn find_common_attributes(&self, children: &[Node]) -> HashMap<String, String> {
        let movable_attrs = Self::get_movable_attributes();
        let mut common_attrs: Option<HashMap<String, String>> = None;

        // Only consider child elements, not text nodes
        let child_elements: Vec<&Element> = children
            .iter()
            .filter_map(|node| match node {
                Node::Element(elem) => Some(elem),
                _ => None,
            })
            .collect();

        if child_elements.len() < 2 {
            return HashMap::new();
        }

        for elem in child_elements {
            let mut elem_attrs = HashMap::new();

            // Collect movable attributes from this element
            for (name, value) in &elem.attributes {
                if movable_attrs.contains(name.as_str()) {
                    elem_attrs.insert(name.clone(), value.clone());
                }
            }

            match &common_attrs {
                None => {
                    // First element sets the baseline
                    common_attrs = Some(elem_attrs);
                }
                Some(existing) => {
                    // Keep only attributes that match between elements
                    let mut intersection = HashMap::new();
                    for (name, value) in existing {
                        if let Some(elem_value) = elem_attrs.get(name) {
                            if elem_value == value {
                                intersection.insert(name.clone(), value.clone());
                            }
                        }
                    }
                    common_attrs = Some(intersection);
                }
            }

            // If no common attributes remain, no point continuing
            if common_attrs.as_ref().map_or(true, |attrs| attrs.is_empty()) {
                break;
            }
        }

        common_attrs.unwrap_or_default()
    }

    /// Remove attributes from child elements
    fn remove_attributes_from_children(
        &self,
        children: &mut [Node],
        attrs_to_remove: &HashSet<String>,
    ) {
        for node in children {
            if let Node::Element(elem) = node {
                for attr_name in attrs_to_remove {
                    elem.remove_attr(attr_name);
                }
            }
        }
    }

    /// Add attributes to the parent element
    fn add_attributes_to_parent(
        &self,
        parent: &mut Element,
        attrs_to_add: &HashMap<String, String>,
    ) {
        for (name, value) in attrs_to_add {
            // Only add if the parent doesnPROTECTED_104_static str {
        "moveElemsAttrsToGroup"
    }

    fn description(&self) -> &'static str {
        PROTECTED_30_
    }

    fn validate_params(&self, params: &Value) -> Result<()> {
        Self::parse_config(params)?;
        Ok(())
    }

    fn apply(&self, document: &mut Document) -> Result<()> {
        self.process_element(&mut document.root);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vexy_svgo_core::ast::*;

    fn create_test_element(tag: &str, attrs: Vec<(&str, &str)>) -> Element<'static> {
        let mut element = Element::new(tag.to_string());
        for (name, value) in attrs {
            element.set_attr(name, value);
        }
        element
    }

    #[test]
    fn test_plugin_info() {
        let plugin = MoveElemsAttrsToGroupPlugin::new();
        assert_eq!(plugin.name(), "moveElemsAttrsToGroup");
        assert_eq!(
            plugin.description(),
            "move common attributes from elements to their group"
        );
    }

    #[test]
    fn test_param_validation() {
        let plugin = MoveElemsAttrsToGroupPlugin::new();

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
    fn test_move_common_fill_attribute() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![]);
        group.children = vec![
            Node::Element(create_test_element(
                "rect",
                vec![("fill", "red"), ("x", "0")],
            )),
            Node::Element(create_test_element(
                "circle",
                vec![("fill", "red"), ("r", "5")],
            )),
        ];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group)];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Check that the group now has the fill attribute
        if let Node::Element(ref group) = document.root.children[0] {
            assert_eq!(group.attr("fill").map(|s| s.as_str()), Some("red"));

            // Check that children no longer have the fill attribute
            for child in &group.children {
                if let Node::Element(elem) = child {
                    assert!(!elem.has_attr("fill"));
                }
            }
        }
    }

    #[test]
    fn test_no_change_when_attributes_differ() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![]);
        group.children = vec![
            Node::Element(create_test_element("rect", vec![("fill", "red")])),
            Node::Element(create_test_element("circle", vec![("fill", "blue")])),
        ];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group.clone())];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Group should not have fill attribute
        if let Node::Element(ref result_group) = document.root.children[0] {
            assert!(!result_group.has_attr("fill"));

            // Children should still have their original fill attributes
            if let Node::Element(ref rect) = result_group.children[0] {
                assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("red"));
            }
            if let Node::Element(ref circle) = result_group.children[1] {
                assert_eq!(circle.attr("fill").map(|s| s.as_str()), Some("blue"));
            }
        }
    }

    #[test]
    fn test_multiple_common_attributes() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![]);
        group.children = vec![
            Node::Element(create_test_element(
                "rect",
                vec![("fill", "red"), ("stroke", "blue"), ("opacity", "0.5")],
            )),
            Node::Element(create_test_element(
                "circle",
                vec![("fill", "red"), ("stroke", "blue"), ("opacity", "0.5")],
            )),
        ];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group)];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Check that the group has all common attributes
        if let Node::Element(ref group) = document.root.children[0] {
            assert_eq!(group.attr("fill").map(|s| s.as_str()), Some("red"));
            assert_eq!(group.attr("stroke").map(|s| s.as_str()), Some("blue"));
            assert_eq!(group.attr("opacity").map(|s| s.as_str()), Some("0.5"));

            // Check that children no longer have these attributes
            for child in &group.children {
                if let Node::Element(elem) = child {
                    assert!(!elem.has_attr("fill"));
                    assert!(!elem.has_attr("stroke"));
                    assert!(!elem.has_attr("opacity"));
                }
            }
        }
    }

    #[test]
    fn test_group_already_has_attribute() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![("fill", "green")]);
        group.children = vec![
            Node::Element(create_test_element("rect", vec![("fill", "red")])),
            Node::Element(create_test_element("circle", vec![("fill", "red")])),
        ];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group)];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Group should keep its original fill attribute
        if let Node::Element(ref group) = document.root.children[0] {
            assert_eq!(group.attr("fill").map(|s| s.as_str()), Some("green"));

            // Children should not have fill attribute removed since group already had different value
            for child in &group.children {
                if let Node::Element(elem) = child {
                    assert!(!elem.has_attr("fill"));
                }
            }
        }
    }

    #[test]
    fn test_single_child_no_change() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![]);
        group.children = vec![Node::Element(create_test_element(
            "rect",
            vec![("fill", "red")],
        ))];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group.clone())];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // No changes should be made with only one child
        if let Node::Element(ref result_group) = document.root.children[0] {
            assert!(!result_group.has_attr("fill"));

            if let Node::Element(ref rect) = result_group.children[0] {
                assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("red"));
            }
        }
    }
}
            }
        }
    }

    #[test]
    fn test_multiple_common_attributes() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![]);
        group.children = vec![
            Node::Element(create_test_element(
                "rect",
                vec![("fill", "red"), ("stroke", "blue"), ("opacity", "0.5")],
            )),
            Node::Element(create_test_element(
                "circle",
                vec![("fill", "red"), ("stroke", "blue"), ("opacity", "0.5")],
            )),
        ];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group)];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Check that the group has all common attributes
        if let Node::Element(ref group) = document.root.children[0] {
            assert_eq!(group.attr("fill").map(|s| s.as_str()), Some("red"));
            assert_eq!(group.attr("stroke").map(|s| s.as_str()), Some("blue"));
            assert_eq!(group.attr("opacity").map(|s| s.as_str()), Some("0.5"));

            // Check that children no longer have these attributes
            for child in &group.children {
                if let Node::Element(elem) = child {
                    assert!(!elem.has_attr("fill"));
                    assert!(!elem.has_attr("stroke"));
                    assert!(!elem.has_attr("opacity"));
                }
            }
        }
    }

    #[test]
    fn test_group_already_has_attribute() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![("fill", "green")]);
        group.children = vec![
            Node::Element(create_test_element("rect", vec![("fill", "red")])),
            Node::Element(create_test_element("circle", vec![("fill", "red")])),
        ];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group)];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // Group should keep its original fill attribute
        if let Node::Element(ref group) = document.root.children[0] {
            assert_eq!(group.attr("fill").map(|s| s.as_str()), Some("green"));

            // Children should not have fill attribute removed since group already had different value
            for child in &group.children {
                if let Node::Element(elem) = child {
                    assert!(!elem.has_attr("fill"));
                }
            }
        }
    }

    #[test]
    fn test_single_child_no_change() {
        let mut plugin = MoveElemsAttrsToGroupPlugin::new();

        let mut group = create_test_element("g", vec![]);
        group.children = vec![Node::Element(create_test_element(
            "rect",
            vec![("fill", "red")],
        ))];

        let mut document = Document::default();
        document.root.children = vec![Node::Element(group.clone())];

        let result = plugin.apply(&mut document);
        assert!(result.is_ok());

        // No changes should be made with only one child
        if let Node::Element(ref result_group) = document.root.children[0] {
            assert!(!result_group.has_attr("fill"));

            if let Node::Element(ref rect) = result_group.children[0] {
                assert_eq!(rect.attr("fill").map(|s| s.as_str()), Some("red"));
            }
        }
    }
}