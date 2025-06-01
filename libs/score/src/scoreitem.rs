use graphics::{color::Color, rectangle::Rectangle};

#[derive(Debug)]
pub enum ScoreItem {
    BarlineNormal,
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
}

pub type ScoreRectangle = (Rectangle, ScoreItem);
pub type ScoreRectangles = Vec<ScoreRectangle>;
