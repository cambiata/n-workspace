use core::{
    accidental::Accidental,
    barline::BarlineType,
    clef::ClefSignature,
    head::{HeadType, HeadVariant},
    rest::RestType,
};

use graphics::{color::Color, rectangle::Rectangle};

#[derive(Debug, Clone)]
pub enum GlyphItem {
    XRect(Color),
    //
    Barline(BarlineType),
    Notehead(HeadType, HeadVariant),
    Rest(RestType),
    BarRest,
    //
    RestBrevis,
    RestWhole,
    RestHalf,
    RestQuarter,
    RestEighth,
    RestSixteenth,

    Accidental(Accidental),
    Clef(ClefSignature),
    // TieFromResolved(i8),
    // TieFromUnresolved(i8),
}

pub type GlyphRectangle = (Rectangle, GlyphItem);
pub type ComplexGlyphsRectangles = Vec<GlyphRectangle>;
pub type PartGlyphsRectangles = Vec<ComplexGlyphsRectangles>;
pub type SysitemGlyphsRectangles = Vec<PartGlyphsRectangles>;
