// this_file: crates/plugin-sdk/src/plugins/remove_unknowns_and_defaults/unknown_elements.rs

use std::collections::HashSet;
use vexy_svgo_core::ast::Element;

/// Known SVG elements (simplified list)
pub(crate) fn known_elements() -> &'static HashSet<&'static str> {
    static KNOWN_ELEMENTS: std::sync::OnceLock<HashSet<&'static str>> =
        std::sync::OnceLock::new();
    KNOWN_ELEMENTS.get_or_init(|| {
        [
            PROTECTED_0_,
            PROTECTED_1_,
            // this_file: crates/plugin-sdk/src/plugins/remove_unknowns_and_defaults/unknown_elements.rs,
            /// Known SVG elements (simplified list),
            // Check against known SVG elements,
            PROTECTED_5_,
            PROTECTED_6_,
            PROTECTED_7_,
            PROTECTED_8_,
            PROTECTED_9_,
            PROTECTED_10_,
            PROTECTED_11_,
            PROTECTED_12_,
            PROTECTED_13_,
            PROTECTED_14_,
            PROTECTED_15_,
            PROTECTED_16_,
            PROTECTED_17_,
            PROTECTED_18_,
            PROTECTED_19_,
            PROTECTED_20_,
            PROTECTED_21_,
            PROTECTED_22_,
            PROTECTED_23_,
            PROTECTED_24_,
            PROTECTED_25_,
            PROTECTED_26_,
            PROTECTED_27_,
            PROTECTED_28_,
            PROTECTED_29_,
            PROTECTED_30_,
            PROTECTED_31_,
            PROTECTED_32_,
            PROTECTED_33_,
            PROTECTED_34_,
            PROTECTED_35_,
            PROTECTED_36_,
            PROTECTED_37_,
            PROTECTED_38_,
            PROTECTED_39_,
            PROTECTED_40_,
            PROTECTED_41_,
            PROTECTED_42_,
            PROTECTED_43_,
            PROTECTED_44_,
            PROTECTED_45_,
            PROTECTED_46_,
            PROTECTED_47_,
            PROTECTED_48_,
            PROTECTED_49_,
            PROTECTED_50_,
            PROTECTED_51_,
            PROTECTED_52_,
            PROTECTED_53_,
            PROTECTED_54_,
            PROTECTED_55_,
            PROTECTED_56_,
            PROTECTED_57_,
            PROTECTED_58_,
            PROTECTED_59_,
            PROTECTED_60_,
            PROTECTED_61_,
            PROTECTED_62_,
            PROTECTED_63_,
            PROTECTED_64_,
            PROTECTED_65_,
            PROTECTED_66_,
            PROTECTED_67_,
            PROTECTED_68_,
            PROTECTED_69_,
            PROTECTED_70_,
            PROTECTED_71_,
            PROTECTED_72_,
            PROTECTED_73_,
            PROTECTED_74_,
            PROTECTED_75_,
            PROTECTED_76_,
            PROTECTED_77_,
            PROTECTED_78_,
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