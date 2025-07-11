// this_file: crates/plugin-sdk/src/selector.rs

//! SVG element selector matching for plugins.
//!
//! This module enables CSS selector matching on the vexy_svgo AST for SVG elements.
//! It provides wrapper types and trait implementations required by the `selectors` crate,
//! allowing selectors to operate on the vexy_svgo DOM.
//!
//! # Features
//! - Implements all required traits for selector matching on SVG elements.
//! - Uses wrapper types to avoid Rust orphan rule violations when implementing external traits.
//! - Provides a `SvgElement` wrapper for traversing and matching SVG elements.
//! - Exposes `matches_selector` for checking if an SVG element matches a CSS selector.
//! - Exposes `walk_element_tree_with_parent` for tree traversal with parent context.

use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::parser::{Selector, SelectorImpl};
use selectors::{Element as SelectorElement, OpaqueElement};
use std::borrow::Borrow;
use std::fmt;
use vexy_svgo_core::ast::{Element, Node};

// Import PrecomputedHash trait - required for SelectorImpl associated types
use precomputed_hash::PrecomputedHash;

/// Wrapper types to implement required traits for the selectors crate.
///
/// These types wrap `String` to avoid orphan rule violations (E0117) when implementing external traits.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SvgAttrValue(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SvgIdentifier(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SvgLocalName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SvgNamespacePrefix(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SvgNamespaceUrl(pub String);

// Implement required traits for our wrapper types
impl cssparser::ToCss for SvgAttrValue {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl PrecomputedHash for SvgAttrValue {
    fn precomputed_hash(&self) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as u32
    }
}

impl cssparser::ToCss for SvgIdentifier {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl PrecomputedHash for SvgIdentifier {
    fn precomputed_hash(&self) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as u32
    }
}

impl cssparser::ToCss for SvgLocalName {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl PrecomputedHash for SvgLocalName {
    fn precomputed_hash(&self) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as u32
    }
}

impl cssparser::ToCss for SvgNamespacePrefix {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl PrecomputedHash for SvgNamespacePrefix {
    fn precomputed_hash(&self) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as u32
    }
}

impl cssparser::ToCss for SvgNamespaceUrl {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

impl PrecomputedHash for SvgNamespaceUrl {
    fn precomputed_hash(&self) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish() as u32
    }
}

// Implement Borrow<str> for types that need it
impl Borrow<str> for SvgLocalName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for SvgNamespaceUrl {
    fn borrow(&self) -> &str {
        &self.0
    }
}

// Implement From<&str> for our wrapper types as required by selectors crate
impl<'a> From<&'a str> for SvgAttrValue {
    fn from(s: &'a str) -> Self {
        SvgAttrValue(s.to_string())
    }
}

impl<'a> From<&'a str> for SvgIdentifier {
    fn from(s: &'a str) -> Self {
        SvgIdentifier(s.to_string())
    }
}

impl<'a> From<&'a str> for SvgLocalName {
    fn from(s: &'a str) -> Self {
        SvgLocalName(s.to_string())
    }
}

impl<'a> From<&'a str> for SvgNamespacePrefix {
    fn from(s: &'a str) -> Self {
        SvgNamespacePrefix(s.to_string())
    }
}

impl<'a> From<&'a str> for SvgNamespaceUrl {
    fn from(s: &'a str) -> Self {
        SvgNamespaceUrl(s.to_string())
    }
}

/// SVG selector implementation for use with the selectors crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SvgSelectorImpl;

impl SelectorImpl for SvgSelectorImpl {
    type ExtraMatchingData<'a> = ();
    type AttrValue = SvgAttrValue;
    type Identifier = SvgIdentifier;
    type LocalName = SvgLocalName;
    type NamespacePrefix = SvgNamespacePrefix;
    type NamespaceUrl = SvgNamespaceUrl;
    type BorrowedNamespaceUrl = str;
    type BorrowedLocalName = str;

    type NonTSPseudoClass = NonTSPseudoClass;
    type PseudoElement = PseudoElement;
}

/// Non-tree-structural pseudo-class (not used for SVG, but required by selectors crate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonTSPseudoClass {}

impl fmt::Display for NonTSPseudoClass {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl cssparser::ToCss for NonTSPseudoClass {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match *self {}
    }
}

impl selectors::parser::NonTSPseudoClass for NonTSPseudoClass {
    type Impl = SvgSelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        match *self {}
    }

    fn is_user_action_state(&self) -> bool {
        match *self {}
    }
}

/// Pseudo-element (not used for SVG, but required by selectors crate).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoElement {}

impl fmt::Display for PseudoElement {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl cssparser::ToCss for PseudoElement {
    fn to_css<W>(&self, _dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        match *self {}
    }
}

impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = SvgSelectorImpl;
}

/// Wrapper around an SVG `Element` to implement the `selectors::Element` trait.
#[derive(Debug)]
pub struct SvgElement<'a> {
    pub element: &'a Element<'a>,
}

/// Type alias for compatibility with other modules.
pub type SvgElementWrapper<'a> = SvgElement<'a>;

impl<'a> SvgElement<'a> {
    /// Create a new `SvgElement` wrapper from a reference to an `Element`.
    pub fn new(element: &'a Element<'a>) -> Self {
        SvgElement { element }
    }
}

impl<'a> Clone for SvgElement<'a> {
    fn clone(&self) -> Self {
        SvgElement {
            element: self.element,
        }
    }
}

impl<'a> SelectorElement for SvgElement<'a> {
    type Impl = SvgSelectorImpl;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self.element)
    }

    fn apply_selector_flags(&self, _flags: selectors::matching::ElementSelectorFlags) {
        // No-op for SVG elements - we don't need to track selector flags
    }

    fn parent_element(&self) -> Option<Self> {
        None // Simplified implementation for now
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        None // Simplified implementation for now
    }

    fn next_sibling_element(&self) -> Option<Self> {
        None // Simplified implementation for now
    }

    fn first_element_child(&self) -> Option<Self> {
        self.element.children.iter().find_map(|child| {
            if let Node::Element(element) = child {
                Some(SvgElement::new(element))
            } else {
                None
            }
        })
    }

    fn is_html_element_in_html_document(&self) -> bool {
        false
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        self.element.name == local_name
    }

    fn has_namespace(&self, _namespace: &str) -> bool {
        true // SVG elements are in the SVG namespace
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.element.name == other.element.name
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&SvgNamespaceUrl>,
        local_name: &SvgLocalName,
        operation: &AttrSelectorOperation<&SvgAttrValue>,
    ) -> bool {
        // Only match attributes without namespace for now
        if !matches!(ns, NamespaceConstraint::Specific(ns_val) if ns_val.0.is_empty())
            && !matches!(ns, NamespaceConstraint::Any)
        {
            return false;
        }

        if let Some(attr_value) = self.element.attr(&local_name.0) {
            match operation {
                AttrSelectorOperation::Exists => true,
                AttrSelectorOperation::WithValue {
                    operator,
                    case_sensitivity,
                    value,
                } => {
                    let case_insensitive =
                        matches!(case_sensitivity, CaseSensitivity::AsciiCaseInsensitive);

                    match operator {
                        selectors::attr::AttrSelectorOperator::Equal => {
                            if case_insensitive {
                                attr_value.to_lowercase() == value.0.to_lowercase()
                            } else {
                                attr_value == &value.0
                            }
                        }
                        selectors::attr::AttrSelectorOperator::Includes => {
                            let values: Vec<&str> = attr_value.split_whitespace().collect();
                            if case_insensitive {
                                values
                                    .iter()
                                    .any(|v| v.to_lowercase() == value.0.to_lowercase())
                            } else {
                                values.contains(&value.0.as_str())
                            }
                        }
                        selectors::attr::AttrSelectorOperator::DashMatch => {
                            if case_insensitive {
                                let attr_lower = attr_value.to_lowercase();
                                let expected_lower = value.0.to_lowercase();
                                attr_lower == expected_lower
                                    || attr_lower.starts_with(&format!("{}-", expected_lower))
                            } else {
                                attr_value == &value.0
                                    || attr_value.starts_with(&format!("{}-", value.0))
                            }
                        }
                        selectors::attr::AttrSelectorOperator::Prefix => {
                            if case_insensitive {
                                attr_value
                                    .to_lowercase()
                                    .starts_with(&value.0.to_lowercase())
                            } else {
                                attr_value.starts_with(&value.0)
                            }
                        }
                        selectors::attr::AttrSelectorOperator::Suffix => {
                            if case_insensitive {
                                attr_value.to_lowercase().ends_with(&value.0.to_lowercase())
                            } else {
                                attr_value.ends_with(&value.0)
                            }
                        }
                        selectors::attr::AttrSelectorOperator::Substring => {
                            if case_insensitive {
                                attr_value.to_lowercase().contains(&value.0.to_lowercase())
                            } else {
                                attr_value.contains(&value.0)
                            }
                        }
                    }
                }
            }
        } else {
            false
        }
    }

    fn match_non_ts_pseudo_class(
        &self,
        _pc: &NonTSPseudoClass,
        _context: &mut selectors::matching::MatchingContext<SvgSelectorImpl>,
    ) -> bool {
        match *_pc {}
    }

    fn match_pseudo_element(
        &self,
        _pe: &PseudoElement,
        _context: &mut selectors::matching::MatchingContext<SvgSelectorImpl>,
    ) -> bool {
        match *_pe {}
    }

    fn is_link(&self) -> bool {
        false
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn has_id(&self, id: &SvgIdentifier, _case_sensitivity: CaseSensitivity) -> bool {
        self.element.attr("id") == Some(&id.0)
    }

    fn has_class(&self, name: &SvgIdentifier, _case_sensitivity: CaseSensitivity) -> bool {
        if let Some(class_attr) = self.element.attr("class") {
            class_attr.split_whitespace().any(|c| c == name.0)
        } else {
            false
        }
    }

    fn imported_part(&self, _name: &SvgIdentifier) -> Option<SvgIdentifier> {
        None
    }

    fn is_part(&self, _name: &SvgIdentifier) -> bool {
        false
    }

    fn has_custom_state(&self, _name: &SvgIdentifier) -> bool {
        false
    }

    fn add_element_unique_hashes(
        &self,
        _filter: &mut selectors::bloom::CountingBloomFilter<selectors::bloom::BloomStorageU8>,
    ) -> bool {
        true
    }

    fn is_empty(&self) -> bool {
        self.element.children.is_empty()
    }

    fn is_root(&self) -> bool {
        self.element.name == "svg"
    }
}

/// Walk the SVG element tree, visiting each element with its parent context.
///
/// # Arguments
/// * `element` - The current SVG element.
/// * `parent` - The parent SVG element, or `None` if at the root.
/// * `visitor` - A mutable closure called for each element and its parent.
///
/// This function is used by other modules for traversing SVG elements.
///
/// # Implementation Note
/// Uses dynamic dispatch instead of monomorphization to avoid recursion limit issues
/// during compilation with complex closure types.
pub fn walk_element_tree_with_parent<'a>(
    element: &Element<'a>,
    parent: Option<&Element<'a>>,
    visitor: &mut dyn FnMut(&Element<'a>, Option<&Element<'a>>),
) {
    visitor(element, parent);

    for child in &element.children {
        if let Node::Element(child_element) = child {
            walk_element_tree_with_parent(child_element, Some(element), visitor);
        }
    }
}

/// Check if a CSS selector matches an SVG element.
///
/// # Arguments
/// * `element` - The SVG element to test.
/// * `selector` - The parsed CSS selector.
///
/// # Returns
/// `true` if the selector matches the element, `false` otherwise.
pub fn matches_selector(element: &Element<'_>, selector: &Selector<SvgSelectorImpl>) -> bool {
    use selectors::matching::SelectorCaches;
    use selectors::matching::{
        MatchingForInvalidation, MatchingMode, NeedsSelectorFlags, QuirksMode,
    };

    let svg_element = SvgElement::new(element);
    let mut selector_caches = SelectorCaches::default();
    let mut context = selectors::matching::MatchingContext::new(
        MatchingMode::Normal,
        None,
        &mut selector_caches,
        QuirksMode::NoQuirks,
        NeedsSelectorFlags::No,
        MatchingForInvalidation::No,
    );

    selectors::matching::matches_selector(selector, 0, None, &svg_element, &mut context)
}
