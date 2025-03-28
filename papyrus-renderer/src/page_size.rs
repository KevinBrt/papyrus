use pdf_writer::{Pdf, Rect, Ref};

pub struct PageSize {
    width: f32,
    height: f32,
}

impl PageSize {
    // Formats standards en points (1 point = 1/72 pouce)
    pub const A4: PageSize = PageSize {
        width: 595.0,  // 210mm
        height: 842.0, // 297mm
    };

    pub const A3: PageSize = PageSize {
        width: 842.0,   // 297mm
        height: 1191.0, // 420mm
    };

    pub const LETTER: PageSize = PageSize {
        width: 612.0,  // 8.5"
        height: 792.0, // 11"
    };

    pub fn to_media_box(&self) -> Rect {
        // Commence à (0,0) et va jusqu'à (width, height)
        Rect::new(0.0, 0.0, self.width, self.height)
    }

    pub fn to_landscape(&self) -> Rect {
        // Inverse width et height pour le mode paysage
        Rect::new(0.0, 0.0, self.height, self.width)
    }
}
