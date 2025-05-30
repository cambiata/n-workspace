use rusttype::Font;
use std::cell::RefCell;

#[allow(unused)]
#[derive(Debug)]
pub struct FontContext<'a> {
    pub music_font: RefCell<Font<'a>>,
    pub sansserif_font: RefCell<Font<'a>>,
}

#[allow(unused)]
impl<'a> FontContext<'a> {
    pub fn new() -> &'static FontContext<'a> {
        let cx = FontContext {
            music_font: RefCell::new(Font::try_from_bytes(include_bytes!("../assets/MTF-Cadence-Fin.ttf") as &[u8]).expect("Error constructing Font")),
            sansserif_font: RefCell::new(Font::try_from_bytes(include_bytes!("../assets/OpenSans-Regular.ttf") as &[u8]).expect("Error constructing Font")),
        };
        Box::leak(Box::new(cx))
    }
}
