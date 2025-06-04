use graphics::graphicitem::GraphicItem;
use rusttype::Font;
use std::cell::RefCell;

use crate::utils::{get_dimensions, get_items};

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

    pub fn get_music_string_dimensions(&self, scale: f32, text: &str) -> (f32, f32) {
        let font = &self.music_font.borrow();
        get_dimensions(font, scale, text)
    }

    pub fn get_music_string_items(&self, scale: f32, text: &str) -> Vec<GraphicItem> {
        let font = &self.music_font.borrow();
        get_items(font, scale, text)
    }

    pub fn get_sansserif_string_dimensions(&self, scale: f32, text: &str) -> (f32, f32) {
        let font = &self.sansserif_font.borrow();
        get_dimensions(font, scale, text)
    }

    pub fn get_sansserif_string_items(&self, scale: f32, text: &str) -> Vec<GraphicItem> {
        let font = &self.sansserif_font.borrow();
        get_items(font, scale, text)
    }
}
