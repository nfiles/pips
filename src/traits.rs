use std::collections::HashMap;

pub trait Rollable {
    fn roll(&self) -> i32;
    fn plot(&self) -> HashMap<i32, i32>;
}

// pub struct ProbabilityPlot {
//     outcomes: i32,
//     plot: HashMap<i32, i32>,
// }
