// this_file: crates/core/src/parser/streaming.rs

use crate::ast::{Document, Element, Node};
use crate::parser::config::StreamingConfig;
use crate::parser::entities::{expand_entities_in_text, parse_entities_from_doctype};
use crate::parser::error::ParseResult;
use crate::parser::util::TEXT_ELEMENTS;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;
use std::io::BufRead;

/// Streaming parser for large SVG documents
pub struct StreamingParser<R: BufRead> {
    reader: Reader<R>,
    config: StreamingConfig,
    preserve_whitespace: bool,
    preserve_comments: bool,
    expand_entities: bool,
    #[allow(dead_code)]
    file_path: Option<String>,
    current_depth: usize,
    bytes_processed: usize,
}

impl<R: BufRead> StreamingParser<R> {
    pub fn new(reader: R, config: StreamingConfig, preserve_whitespace: bool, preserve_comments: bool, expand_entities: bool, file_path: Option<String>) -> Self {
        let mut xml_reader = Reader::from_reader(reader);
        xml_reader.config_mut().expand_empty_elements = true;
        xml_reader.config_mut().trim_text_start = !preserve_whitespace;
        xml_reader.config_mut().trim_text_end = !preserve_whitespace;
        xml_reader.config_mut().check_end_names = true;
        
        Self {
            reader: xml_reader,
            config,
            preserve_whitespace,
            preserve_comments,
            expand_entities,
            file_path,
            current_depth: 0,
            bytes_processed: 0,
        }
    }

    /// Parse the document with streaming optimizations
    pub fn parse(&mut self) -> ParseResult<Document<'static>> {
        let mut document = Document::new();
        let mut element_stack: Vec<Element<'static>> = Vec::new();
        let mut current_element: Option<Element<'static>> = None;
        let mut found_root = false;
        let mut entities: HashMap<String, String> = HashMap::new();
        let mut element_name_stack: Vec<String> = Vec::new();
        let mut buf = Vec::with_capacity(self.config.buffer_size);

        loop {
            buf.clear();

            // Check depth limit to prevent stack overflow
            if self.current_depth > self.config.max_depth {
                return Err(crate::parser::error::ParseError::StructureError(
                    format!("Maximum parsing depth exceeded: {}", self.config.max_depth),
                ));
            }

            match self.reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    self.current_depth += 1;
                    let element = self.parse_start_element(e, &entities)?;
                    element_name_stack.push(element.name.to_string());

                    if current_element.is_none() {
                        current_element = Some(element);
                    } else {
                        if let Some(elem) = current_element.take() {
                            element_stack.push(elem);
                        }
                        current_element = Some(element);
                    }
                }
                Ok(Event::End(_)) => {
                    self.current_depth = self.current_depth.saturating_sub(1);
                    element_name_stack.pop();

                    if let Some(finished_element) = current_element.take() {
                        if let Some(mut parent) = element_stack.pop() {
                            parent.add_child(Node::Element(finished_element));
                            current_element = Some(parent);
                        } else {
                            document.root = finished_element;
                            found_root = true;
                        }
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let element = self.parse_start_element(e, &entities)?;

                    if let Some(ref mut parent) = current_element {
                        parent.add_child(Node::Element(element));
                    } else {
                        document.root = element;
                        found_root = true;
                    }
                }
                Ok(Event::Text(ref e)) => {
                    let text_str = match std::str::from_utf8(e.as_ref()) {
                        Ok(s) => s,
                        Err(_) => continue, // Skip invalid UTF-8 in streaming mode
                    };

                    // Check for large text nodes
                    if let Some(threshold) = self.config.large_text_threshold {
                        if text_str.len() > threshold {
                            // Skip or truncate very large text nodes to preserve memory
                            if let Some(ref mut element) = current_element {
                                let truncated = format!(
                                    "{}... [truncated {} bytes]",
                                    &text_str[..threshold.min(100)],
                                    text_str.len() - threshold.min(100),
                                );
                                element.add_child(Node::Text(truncated));
                            }
                            continue;
                        }
                    }

                    let text = quick_xml::escape::unescape(text_str)
                        .unwrap_or(std::borrow::Cow::Borrowed(text_str));
                    let mut text_content = text.into_owned();

                    if self.expand_entities && !entities.is_empty() {
                        text_content = expand_entities_in_text(&text_content, &entities);
                    }

                    let should_preserve_whitespace = self.preserve_whitespace
                        || element_name_stack
                            .last()
                            .map(|name| TEXT_ELEMENTS.contains(name.as_str()))
                            .unwrap_or(false);

                    if should_preserve_whitespace || !text_content.trim().is_empty() {
                        if let Some(ref mut element) = current_element {
                            element.add_child(Node::Text(text_content));
                        }
                    }
                }
                Ok(Event::Comment(ref e)) => {
                    if self.preserve_comments {
                        let comment = match std::str::from_utf8(e.as_ref()) {
                            Ok(s) => s.to_string(),
                            Err(_) => continue, // Skip invalid UTF-8
                        };

                        if let Some(ref mut element) = current_element {
                            element.add_child(Node::Comment(comment));
                        } else if !found_root {
                            document.prologue.push(Node::Comment(comment));
                        } else {
                            document.epilogue.push(Node::Comment(comment));
                        }
                    }
                }
                Ok(Event::CData(ref e)) => {
                    let cdata = match std::str::from_utf8(e.as_ref()) {
                        Ok(s) => s.to_string(),
                        Err(_) => continue, // Skip invalid UTF-8
                    };

                    if let Some(ref mut element) = current_element {
                        element.add_child(Node::CData(cdata));
                    }
                }
                Ok(Event::PI(ref e)) => {
                    let pi_data = match std::str::from_utf8(e.as_ref()) {
                        Ok(s) => s,
                        Err(_) => continue, // Skip invalid UTF-8
                    };

                    let parts: Vec<&str> = pi_data.splitn(2, ' ').collect();
                    let target = parts[0].to_string();
                    let data = parts.get(1).unwrap_or(&"").to_string();

                    let pi_node = Node::ProcessingInstruction { target, data };

                    if let Some(ref mut element) = current_element {
                        element.add_child(pi_node);
                    } else if !found_root {
                        document.prologue.push(pi_node);
                    } else {
                        document.epilogue.push(pi_node);
                    }
                }
                Ok(Event::Decl(ref e)) => {
                    if let Ok(version) = e.version() {
                        document.metadata.version =
                            Some(String::from_utf8_lossy(&version).to_string());
                    }
                    if let Some(Ok(enc)) = e.encoding() {
                        document.metadata.encoding =
                            Some(String::from_utf8_lossy(&enc).to_string());
                    }
                }
                Ok(Event::DocType(ref e)) => {
                    let doctype = match std::str::from_utf8(e.as_ref()) {
                        Ok(s) => s.to_string(),
                        Err(_) => continue, // Skip invalid UTF-8
                    };

                    if self.expand_entities {
                        parse_entities_from_doctype(&doctype, &mut entities);
                    }

                    if !found_root {
                        document.prologue.push(Node::DocType(doctype));
                    }
                }
                Ok(Event::Eof) => break,
                Ok(Event::GeneralRef(_)) => {
                    // Ignore general references in streaming mode
                }
                Err(e) => {
                    // In streaming mode, we're more tolerant of errors
                    // but still track position for debugging
                    let byte_pos = self.reader.buffer_position();
                    return Err(crate::parser::error::ParseError::StructureError(
                        format!("Streaming parse error at byte {byte_pos}: {e}"),
                    ));
                }
            }

            // Track bytes processed for progress reporting
            self.bytes_processed = self.reader.buffer_position() as usize;
        }

        if current_element.is_none() && document.root.name.is_empty() {
            return Err(crate::parser::error::ParseError::StructureError(
                "No root element found in document".to_string(),
            ));
        }

        // Optimize memory usage after parsing
        if !self.config.lazy_loading {
            document.root.optimize_memory();
        }

        Ok(document)
    }

    /// Get the number of bytes processed so far
    pub fn bytes_processed(&self) -> usize {
        self.bytes_processed
    }

    /// Get the current parsing depth
    pub fn current_depth(&self) -> usize {
        self.current_depth
    }

    // Helper methods from the main parser
    fn parse_start_element(
        &self,
        start: &quick_xml::events::BytesStart<'_>,
        entities: &HashMap<String, String>,
    ) -> ParseResult<Element<'static>> {
        let name = match std::str::from_utf8(start.name().as_ref()) {
            Ok(n) => n.to_string(),
            Err(_) => "unknown".to_string(), // More tolerant in streaming mode
        };

        let mut element = if self.config.lazy_loading {
            Element::new(std::borrow::Cow::Owned(name))
        } else {
            // Pre-allocate capacity based on typical SVG element patterns
            let expected_children = match name.as_str() {
                "svg" | "g" | "defs" => 10,     // Container elements
                "path" | "rect" | "circle" => 0, // Leaf elements
                _ => 2,                          // Default
            };
            Element::with_capacity(std::borrow::Cow::Owned(name), expected_children)
        };

        // Parse attributes with error tolerance
        for attr in start.attributes().flatten() {
            if let (Ok(key), Ok(value)) = (
                std::str::from_utf8(attr.key.as_ref()),
                attr.unescape_value(),
            ) {
                let mut attr_value = value.to_string();

                if self.expand_entities && !entities.is_empty() {
                    attr_value = expand_entities_in_text(&attr_value, entities);
                }

                // Handle namespaces
                if key.starts_with("xmlns") {
                    if key == "xmlns" {
                        element.namespaces.insert("".to_string(), attr_value.clone());
                    } else if let Some(ns_name) = key.strip_prefix("xmlns:") {
                        element.namespaces.insert(ns_name.to_string(), attr_value.clone());
                    }
                } else {
                    element.set_attr(key.to_string(), attr_value);
                }
            }
            // Continue on attribute parsing errors in streaming mode
        }

        Ok(element)
    }
}
