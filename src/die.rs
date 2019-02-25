use rand::Rng;
use std::collections::HashMap;

use crate::traits::Rollable;

#[derive(Clone)]
pub struct Die {
    max: u32,
}

impl Die {
    pub fn new(max: u32) -> Die {
        Die { max }
    }
}

impl Rollable for Die {
    fn roll(&self) -> i32 {
        rand::thread_rng().gen_range(1, self.max as i32 + 1)
    }

    fn plot(&self) -> HashMap<i32, i32> {
        // a single die has an equal chance to roll any of its sides
        (1..self.max as i32 + 1).map(|i| (i, 1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn die_rolls_expected_numbers() {
        let max = 6;
        let num_rolls = 100;
        let die = Die::new(max);
        let expected: HashSet<_> = [1, 2, 3, 4, 5, 6].iter().cloned().collect();

        let actual: HashSet<_> = (1..num_rolls).map(|_| die.roll()).collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn die_plot_is_correct() {
        let max = 6;
        let die = Die::new(max);
        let expected: HashMap<i32, i32> = [(1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1)]
            .iter()
            .cloned()
            .collect();

        let actual = die.plot();

        assert_eq!(expected, actual);
    }
}
