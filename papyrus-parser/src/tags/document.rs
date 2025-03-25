use crate::parser::DocumentNode;

use super::base::{BaseElement, Element};

pub struct DocumentElement {
    base: BaseElement,
}

impl DocumentElement {
    pub fn new(document_node: &DocumentNode) -> Self {
        Self {
            base: BaseElement::new(document_node),
        }
    }
}

impl Element for DocumentElement {}
