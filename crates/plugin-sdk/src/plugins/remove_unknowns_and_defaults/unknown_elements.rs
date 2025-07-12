// this_file: crates/plugin-sdk/src/plugins/remove_unknowns_and_defaults/unknown_elements.rs

use std::collections::HashSet;
use vexy_svgo_core::ast::Element;

/// Known SVG elements (simplified list)
pub(crate) fn known_elements() -> &'static HashSet<&'static str> {
    static KNOWN_ELEMENTS: std::sync::OnceLock<HashSet<&'static str>> =
        std::sync::OnceLock::new();
    KNOWN_ELEMENTS.get_or_init(|| {
        [
            "svg",
            "g",
            "defs",
            "desc",
            "title",
            "metadata",
            "symbol",
            "marker",
            "clipPath",
            "mask",
            "pattern",
            "image",
            "switch",
            "foreignObject",
            "rect",
            "circle",
            "ellipse",
            "line",
            "polyline",
            "polygon",
            "path",
            "text",
            "tspan",
            "textPath",
            "tref",
            "altGlyph",
            "altGlyphDef",
            "altGlyphItem",
            "glyph",
            "glyphRef",
            "font",
            "font-face",
            "font-face-format",
            "font-face-name",
            "font-face-src",
            "font-face-uri",
            "hkern",
            "vkern",
            "missing-glyph",
            "linearGradient",
            "radialGradient",
            "meshgradient",
            "meshrow",
            "meshpatch",
            "stop",
            "animate",
            "animateColor",
            "animateMotion",
            "animateTransform",
            "mpath",
            "set",
            "cursor",
            "filter",
            "feBlend",
            "feColorMatrix",
            "feComponentTransfer",
            "feComposite",
            "feConvolveMatrix",
            "feDiffuseLighting",
            "feDisplacementMap",
            "feDistantLight",
            "feDropShadow",
            "feFlood",
            "feFuncA",
            "feFuncB",
            "feFuncG",
            "feFuncR",
            "feGaussianBlur",
            "feImage",
            "feMerge",
            "feMergeNode",
            "feMorphology",
            "feOffset",
            "fePointLight",
            "feSpecularLighting",
            "feSpotLight",
            "feTile",
            "feTurbulence",
            "use",
        ]
        .into_iter()
        .collect()
    })
}

pub(crate) fn should_remove_unknown_element(element: &Element, unknown_content: bool) -> bool {
    if !unknown_content {
        return false;
    }

    // Skip namespaced elements
    if element.name.contains(':') {
        return false;
    }

    // Check against known SVG elements
    !known_elements().contains(element.name.as_ref())
}