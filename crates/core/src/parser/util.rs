// this_file: crates/core/src/parser/util.rs

use std::collections::HashSet;
use std::sync::LazyLock;

/// Elements where whitespace should be preserved
pub(crate) static TEXT_ELEMENTS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        // Text content elements
        "text",
        "tspan",
        "textPath",
        "altGlyph",
        "tref",
        "glyph",
        "glyphRef",
        "altGlyphDef",
        "altGlyphItem",
        // Other elements that need whitespace preservation
        "pre",
        "title",
        "script",
        "style",
    ])
});