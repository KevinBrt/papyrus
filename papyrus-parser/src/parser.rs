use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;

use crate::tags::base::Element;
use crate::tags::document::DocumentElement;
use crate::tags::page::PageElement;
use crate::tags::text::TextElement;

#[derive(Debug)]
pub struct DocumentNode {
    pub tag: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<DocumentNode>,
}

impl DocumentNode {
    fn new(tag: String) -> Self {
        Self {
            tag,
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }
}

/**
 * Parser struct
 * @param source: String
 */
pub struct Parser {
    source_xml_path: String,
}

impl Parser {
    pub fn new(source_xml_path: String) -> Self {
        Self { source_xml_path }
    }

    /**
     * Take a XML source file and parse it
     * @param source: &str
     * @return: ()
     */
    pub fn parse(&self) -> Box<dyn Element> {
        let file_content: String = fs::read_to_string(&self.source_xml_path).unwrap();
        let mut reader = Reader::from_str(&file_content);
        let mut node_stack: Vec<DocumentNode> = Vec::new();
        let mut root: Option<DocumentNode> = None;

        loop {
            match reader.read_event() {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {
                    let tag = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                    let mut node = DocumentNode::new(tag);

                    // Process attributes
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = String::from_utf8(attr.key.as_ref().to_vec()).unwrap();
                            let value = String::from_utf8(attr.value.as_ref().to_vec()).unwrap();
                            node.attributes.push((key, value));
                        }
                    }

                    node_stack.push(node);
                }
                Ok(Event::End(_)) => {
                    if let Some(completed_node) = node_stack.pop() {
                        if let Some(parent) = node_stack.last_mut() {
                            parent.children.push(completed_node);
                        } else {
                            root = Some(completed_node);
                        }
                    }
                }
                _ => (),
            }
        }

        tag_factory(&root.unwrap_or_else(|| DocumentNode::new("document".to_string())))
    }

    /**
     * Convert the parsed XML to a Taffy layout
     * @param source: &str
     * @return: ()
     */
    pub fn to_taffy(&self) {
        println!("to_taffy fn : {} ", self.source_xml_path);
        // TODO
    }
}

/**
 * Factory function to create a new tag element based on the DocumentNode and tag name
 * @param document_node: &DocumentNode
 * @return: Box<dyn Element>
 */
pub fn tag_factory(document_node: &DocumentNode) -> Box<dyn Element> {
    match document_node.tag.as_str() {
        // Base elmeents
        "document" => Box::new(DocumentElement::new(document_node)),
        "page" => Box::new(PageElement::new(document_node)),

        // Content elements
        "text" => Box::new(TextElement::new(document_node)),

        // Unknown tag
        _ => panic!("Unknown tag: {}", document_node.tag),
    }
}
