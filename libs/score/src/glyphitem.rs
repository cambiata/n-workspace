use core::{accidental::Accidental, barline::BarlineType, clef::ClefSignature};

use graphics::{color::Color, rectangle::Rectangle};

#[derive(Debug, Clone)]
pub enum GlyphItem {
    Barline(BarlineType),
    //
    XRed,
    XBlue,
    XGreen,
    XOrange,
    X(Color),
    //
    HeadBlack,
    HeadWhite,
    HeadWhole,
    //
    BarRest,
    //
    RestBrevis,
    RestWhole,
    RestHalf,
    RestQuarter,
    RestEighth,
    RestSixteenth,
    //
    Accidental(Accidental),
    //
    Clef(ClefSignature),
}

pub type GlyphRectangle = (Rectangle, GlyphItem);
pub type ComplexGlyphsRectangles = Vec<GlyphRectangle>;
pub type PartGlyphsRectangles = Vec<ComplexGlyphsRectangles>;
pub type SysitemGlyphsRectangles = Vec<PartGlyphsRectangles>;
