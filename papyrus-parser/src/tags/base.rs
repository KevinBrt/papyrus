use crate::parser::{DocumentNode, tag_factory};

pub struct BaseElement {
    attr: Vec<(String, String)>,
    children: Vec<Box<dyn Element>>,
}

impl BaseElement {
    pub fn new(document_node: &DocumentNode) -> Self {
        Self {
            attr: document_node.attributes.clone(),
            children: document_node
                .children
                .iter()
                .map(|child| tag_factory(child))
                .collect(),
        }
    }
}

pub trait Element {}
