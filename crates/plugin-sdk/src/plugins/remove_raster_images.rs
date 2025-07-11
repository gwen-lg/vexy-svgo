// this_file: crates/plugin-sdk/src/plugins/remove_raster_images.rs

//! Remove raster images plugin implementation
//!
//! This plugin removes raster image references in <image> elements.
//! It detects and removes images with JPEG, PNG, or GIF formats.
//!
//! Reference: SVGOPROTECTED_72_static str {
        "removeRasterImages"
    }

    fn description(&self) -> &'static str {
        PROTECTED_9_
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
        let plugin = RemoveRasterImagesPlugin::new();
        assert_eq!(plugin.name(), PROTECTED_10_);
        assert_eq!(
            plugin.description(),
            PROTECTED_11_
        );
    }

    #[test]
    fn test_removes_jpeg_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(PROTECTED_12_);

        // Add JPEG image
        let mut image = create_element(PROTECTED_13_);
        image
            .attributes
            .insert(PROTECTED_14_.to_string(), PROTECTED_15_.to_string());
        doc.root.children.push(Node::Element(image));

        // Add SVG image (should be preserved)
        let mut svg_image = create_element(PROTECTED_16_);
        svg_image
            .attributes
            .insert(PROTECTED_17_.to_string(), PROTECTED_18_.to_string());
        doc.root.children.push(Node::Element(svg_image));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that only SVG image remains
        assert_eq!(doc.root.children.len(), 1);
        if let Node::Element(elem) = &doc.root.children[0] {
            assert_eq!(
                elem.attributes.get(PROTECTED_19_),
                Some(&PROTECTED_20_.to_string())
            );
        }
    }

    #[test]
    fn test_removes_png_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(PROTECTED_21_);

        // Add PNG image
        let mut image = create_element(PROTECTED_22_);
        image
            .attributes
            .insert(PROTECTED_23_.to_string(), PROTECTED_24_.to_string());
        doc.root.children.push(Node::Element(image));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that image was removed
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_removes_gif_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(PROTECTED_25_);

        // Add GIF image
        let mut image = create_element(PROTECTED_26_);
        image
            .attributes
            .insert(PROTECTED_27_.to_string(), PROTECTED_28_.to_string());
        doc.root.children.push(Node::Element(image));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that image was removed
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_removes_data_uri_raster_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(PROTECTED_29_);

        // Add data URI JPEG
        let mut image1 = create_element(PROTECTED_30_);
        image1.attributes.insert(
            PROTECTED_31_.to_string(),
            PROTECTED_32_.to_string(),
        );
        doc.root.children.push(Node::Element(image1));

        // Add data URI PNG
        let mut image2 = create_element(PROTECTED_33_);
        image2.attributes.insert(
            // this_file: crates/plugin-sdk/src/plugins/remove_raster_images.rs.to_string(),
            //! Remove raster images plugin implementation.to_string(),
        );
        doc.root.children.push(Node::Element(image2));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that both images were removed
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_removes_svg2_href_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(//!);

        // Add image with SVG2 href attribute
        let mut image = create_element(//! This plugin removes raster image references in <image> elements.);
        image
            .attributes
            .insert(//! It detects and removes images with JPEG, PNG, or GIF formats..to_string(), //!.to_string());
        doc.root.children.push(Node::Element(image));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that image was removed
        assert_eq!(doc.root.children.len(), 0);
    }

    #[test]
    fn test_preserves_vector_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(//! Reference: SVGOPROTECTED_72_static str {);

        // Add various vector image formats
        let mut svg_image = create_element(// Apply plugin);
        svg_image
            .attributes
            .insert(// Check that non-image elements were preserved.to_string(), // Apply plugin to empty document.to_string());
        doc.root.children.push(Node::Element(svg_image));

        let mut pdf_image = create_element(// Empty object is valid);
        pdf_image
            .attributes
            .insert(// Null is valid.to_string(), // Non-object is invalid.to_string());
        doc.root.children.push(Node::Element(pdf_image));

        let mut data_svg = create_element(// Use parameterized testing framework for SVGO fixture tests);
        data_svg.attributes.insert(
            // Check that all vector images were preserved.to_string(),
            // Create nested structure.to_string(),
        );
        doc.root.children.push(Node::Element(data_svg));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that all vector images were preserved
        assert_eq!(doc.root.children.len(), 3);
    }

    #[test]
    fn test_removes_nested_raster_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(// Apply plugin);

        // Create nested structure
        let mut group = create_element(// Check nested removal);

        let mut image1 = create_element(// First child should be rect (image removed));
        image1
            .attributes
            .insert(// Check pattern has no image.to_string(), // Add elements with raster-like hrefs that aren't images.to_string());
        group.children.push(Node::Element(image1));

        let rect = create_element(// Apply plugin);
        group.children.push(Node::Element(rect));

        let mut defs = create_element(// Check that non-image elements were preserved);
        let mut pattern = create_element(// Apply plugin to empty document);
        let mut image2 = create_element(// Empty object is valid);
        image2
            .attributes
            .insert(// Null is valid.to_string(), // Non-object is invalid.to_string());
        pattern.children.push(Node::Element(image2));
        defs.children.push(Node::Element(pattern));
        group.children.push(Node::Element(defs));

        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check nested removal
        if let Node::Element(g) = &doc.root.children[0] {
            // First child should be rect (image removed)
            if let Node::Element(r) = &g.children[0] {
                assert_eq!(r.name, // Use parameterized testing framework for SVGO fixture tests);
            }

            // Check pattern has no image
            if let Node::Element(d) = &g.children[1] {
                if let Node::Element(p) = &d.children[0] {
                    assert_eq!(p.children.len(), 0);
                }
            }
        }
    }

    #[test]
    fn test_ignores_non_image_elements() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element(PROTECTED_62_);

        // Add elements with raster-like hrefs that aren't images
        let mut use_elem = create_element("use");
        use_elem
            .attributes
            .insert("xlink:href".to_string(), "icon.png".to_string());
        doc.root.children.push(Node::Element(use_elem));

        let mut a_elem = create_element("a");
        a_elem
            .attributes
            .insert("href".to_string(), "photo.jpg".to_string());
        doc.root.children.push(Node::Element(a_elem));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that non-image elements were preserved
        assert_eq!(doc.root.children.len(), 2);
    }

    #[test]
    fn test_empty_document() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin to empty document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveRasterImagesPlugin::new();

        // Empty object is valid
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Null is valid
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Non-object is invalid
        assert!(plugin.validate_params(&json!("invalid")).is_err());
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveRasterImagesPlugin, "removeRasterImages");

        // Check that all vector images were preserved
        assert_eq!(doc.root.children.len(), 3);
    }

    #[test]
    fn test_removes_nested_raster_images() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Create nested structure
        let mut group = create_element("g");

        let mut image1 = create_element("image");
        image1
            .attributes
            .insert("xlink:href".to_string(), "nested.jpg".to_string());
        group.children.push(Node::Element(image1));

        let rect = create_element("rect");
        group.children.push(Node::Element(rect));

        let mut defs = create_element("defs");
        let mut pattern = create_element("pattern");
        let mut image2 = create_element("image");
        image2
            .attributes
            .insert("href".to_string(), "pattern.png".to_string());
        pattern.children.push(Node::Element(image2));
        defs.children.push(Node::Element(pattern));
        group.children.push(Node::Element(defs));

        doc.root.children.push(Node::Element(group));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check nested removal
        if let Node::Element(g) = &doc.root.children[0] {
            // First child should be rect (image removed)
            if let Node::Element(r) = &g.children[0] {
                assert_eq!(r.name, "rect");
            }

            // Check pattern has no image
            if let Node::Element(d) = &g.children[1] {
                if let Node::Element(p) = &d.children[0] {
                    assert_eq!(p.children.len(), 0);
                }
            }
        }
    }

    #[test]
    fn test_ignores_non_image_elements() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Add elements with raster-like hrefs that aren't images
        let mut use_elem = create_element("use");
        use_elem
            .attributes
            .insert("xlink:href".to_string(), "icon.png".to_string());
        doc.root.children.push(Node::Element(use_elem));

        let mut a_elem = create_element("a");
        a_elem
            .attributes
            .insert("href".to_string(), "photo.jpg".to_string());
        doc.root.children.push(Node::Element(a_elem));

        // Apply plugin
        plugin.apply(&mut doc).unwrap();

        // Check that non-image elements were preserved
        assert_eq!(doc.root.children.len(), 2);
    }

    #[test]
    fn test_empty_document() {
        let plugin = RemoveRasterImagesPlugin::new();

        let mut doc = Document::new();
        doc.root = create_element("svg");

        // Apply plugin to empty document
        let result = plugin.apply(&mut doc);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parameter_validation() {
        let plugin = RemoveRasterImagesPlugin::new();

        // Empty object is valid
        assert!(plugin.validate_params(&json!({})).is_ok());

        // Null is valid
        assert!(plugin.validate_params(&Value::Null).is_ok());

        // Non-object is invalid
        assert!(plugin.validate_params(&json!("invalid")).is_err());
    }
}

// Use parameterized testing framework for SVGO fixture tests
crate::plugin_fixture_tests!(RemoveRasterImagesPlugin, "removeRasterImages");
