use crate::parser::DocumentNode;

use super::base::{BaseElement, Element};

pub struct TextElement {
    base: BaseElement,
}

impl TextElement {
    pub fn new(document_node: &DocumentNode) -> Self {
        Self {
            base: BaseElement::new(document_node),
        }
    }
}

impl Element for TextElement {}
