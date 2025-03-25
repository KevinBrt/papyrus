use crate::parser::DocumentNode;

use super::base::{BaseElement, Element};

pub struct PageElement {
    base: BaseElement,
}

impl PageElement {
    pub fn new(document_node: &DocumentNode) -> Self {
        Self {
            base: BaseElement::new(document_node),
        }
    }
}

impl Element for PageElement {}
