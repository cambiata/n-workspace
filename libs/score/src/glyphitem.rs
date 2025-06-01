use graphics::{color::Color, rectangle::Rectangle};

#[derive(Debug)]
pub enum GlyphItem {
    BarlineNormal,
    BarlineDouble,
    BarlineFinal,
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
    RestBrevis,
    RestWhole,
    RestHalf,
    RestQuarter,
    RestEighth,
    RestSixteenth,
    //
    AccidentalSharp,
    AccidentalFlat,
    AccidentalNatural,
    AccidentalDoubleSharp,
    AccidentalDoubleFlat,
    //
    ClefTreble,
    ClefBass,
    ClefAlto,
    ClefTenor,
    ClefPercussion,
}

pub type GlyphRectangle = (Rectangle, GlyphItem);
pub type GlyphsRectangles = Vec<GlyphRectangle>;
