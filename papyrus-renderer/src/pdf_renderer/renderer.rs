use pdf_writer::{Pdf, Ref, writers::Page as WriterPage};

use crate::PageSize;

use super::{Page, PageOptions};

pub struct PdfRenderer {
    pdf: Pdf,
    catalog_id: Ref,
    page_tree_id: Ref,
    page_ids: Vec<Ref>,
    next_id: Ref,
    target_path: String,
}

impl PdfRenderer {
    /**
     * Create a new PdfRenderer
     * @param target_path: &str
     * @return PdfRenderer
     */
    pub fn new(target_path: &str) -> Self {
        let mut pdf = Pdf::new();
        let catalog_id = Ref::new(1);
        let page_tree_id = Ref::new(2);

        // init catalog and page_tree
        pdf.catalog(catalog_id).pages(page_tree_id);

        Self {
            pdf,
            catalog_id,
            page_tree_id,
            page_ids: Vec::new(),
            next_id: Ref::new(3),
            target_path: target_path.to_string(),
        }
    }

    /**
     * Generate a new ref for the PDF and increment the next_id
     * @return Ref
     */
    fn new_ref(&mut self) -> Ref {
        let new_ref = self.next_id.bump();
        new_ref
    }

    /**
     * Save the pdf to target path
     *
     * @param path: &str
     * @return Result<(), String>
     */
    pub fn save(mut self) -> std::io::Result<()> {
        // Add page tree to catalog with all pages ids
        self.pdf
            .pages(self.page_tree_id)
            .kids(self.page_ids.clone())
            .count(self.page_ids.len() as i32);

        std::fs::write(self.target_path.clone(), self.pdf.finish())?;
        Ok(())
    }

    /**
     * Add a new page to the PDF
     * @return Result<(), String>
     */
    pub fn add_page(&mut self) -> Result<(), String> {
        let page_id = self.new_ref();
        self.page_ids.push(page_id);

        self.pdf
            .page(page_id)
            .parent(self.page_tree_id)
            .media_box(PageSize::A4.to_media_box());

        Ok(())
    }
}
