use crate::duration::{NoteDuration, SumDuration};

#[derive(Debug, Clone)]

pub enum TimeSignature {
    None,
    TimeSignature(TimeNominator, TimeDenominator),
    CommonFourFour,
    AllaBreveTwoTwo,
}

#[derive(Debug, Clone)]
pub enum TimeNominator {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Eleven,
    Tweleve,
}

#[derive(Debug, Clone)]
pub enum TimeDenominator {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}

impl TimeSignature {
    pub fn get_duration(&self) -> SumDuration {
        match self {
            TimeSignature::None => NoteDuration::D1 as usize, // Default to 1 if no time signature is set
            TimeSignature::TimeSignature(nominator, denominator) => {
                let den: usize = match denominator {
                    TimeDenominator::One => NoteDuration::D1 as usize,
                    TimeDenominator::Two => NoteDuration::D2 as usize,
                    TimeDenominator::Four => NoteDuration::D4 as usize,
                    TimeDenominator::Eight => NoteDuration::D8 as usize,
                    TimeDenominator::Sixteen => NoteDuration::D16 as usize,
                };
                let nom = match nominator {
                    TimeNominator::One => 1,
                    TimeNominator::Two => 2,
                    TimeNominator::Three => 3,
                    TimeNominator::Four => 4,
                    TimeNominator::Five => 5,
                    TimeNominator::Six => 6,
                    TimeNominator::Seven => 7,
                    TimeNominator::Eight => 8,
                    TimeNominator::Nine => 9,
                    TimeNominator::Eleven => 11,
                    TimeNominator::Tweleve => 12,
                };
                den * nom
            }
            TimeSignature::CommonFourFour => NoteDuration::D1 as usize,
            TimeSignature::AllaBreveTwoTwo => NoteDuration::D1 as usize,
        }
    }
}
