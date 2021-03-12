use std::collections::HashMap;

/// The result of a roll
pub type RollResult = i32;

/// The likelihood of a certain roll result
pub type Chance = f32;

/// table of the percent likelihood of possible outcomes
pub type PlotTable = HashMap<RollResult, Chance>;

/// Plot for a roll expression
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PlotResult {
    /// total number of possible outcomes
    pub total: f32,
    /// likelihood of all possible outcomes in this roll
    pub plot: PlotTable,
}

/// Common trait for a roll expression
pub trait Rollable {
    fn roll(&self) -> RollResult;
    fn plot(&self) -> PlotResult;
}
