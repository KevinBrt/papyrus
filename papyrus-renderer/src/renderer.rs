use crate::pdf_renderer::PdfRenderer;

pub fn render() -> Result<(), String> {
    println!("Rendering...");
    let mut pdf_renderer = PdfRenderer::new("empty.pdf");
    let page = pdf_renderer.add_page()?;

    match pdf_renderer.save() {
        Ok(_) => println!("PDF saved successfully"),
        Err(e) => println!("Error saving PDF: {}", e),
    }
    Ok(())
}
