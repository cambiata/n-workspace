#[derive(Debug)]
#[derive(Debug)]
pub enum TimeSignature {
    TimeSignature(TimeNominator, TimeDenominator),
    CommonFourFour,
    AllaBreveTwoTwo,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum TimeDenominator {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}
