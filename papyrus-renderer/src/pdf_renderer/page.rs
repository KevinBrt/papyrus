use pdf_writer::{Pdf, Ref, writers::Page as WriterPage};

use crate::PageSize;

pub struct PageOptions {
    pub page_size: PageSize,
}

pub struct Page<'a> {
    writer_page: &'a mut WriterPage<'a>,
    page_id: Ref,
    page_tree_id: Ref,
    options: PageOptions,
}

impl<'a> Page<'a> {
    pub fn new(
        writer_page: &'a mut WriterPage<'a>,
        page_tree_id: Ref,
        page_id: Ref,
        options: PageOptions,
    ) -> Self {
        Self {
            writer_page,
            page_id,
            page_tree_id,
            options,
        }
    }
}
