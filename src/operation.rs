use std::collections::HashMap;

use crate::operators::{advantage, disadvantage, multiply, sum};
use crate::traits::Rollable;

type BinaryOperator = fn(left: &i32, right: &i32) -> i32;

pub struct Operation {
    left: Box<dyn Rollable>,
    right: Box<dyn Rollable>,
    combinator: BinaryOperator,
}

impl Operation {
    pub fn sum(left: Box<dyn Rollable>, right: Box<dyn Rollable>) -> Box<dyn Rollable> {
        Box::new(Operation {
            left,
            right,
            combinator: sum,
        })
    }

    pub fn multiply(left: Box<dyn Rollable>, right: Box<dyn Rollable>) -> Box<dyn Rollable> {
        Box::new(Operation {
            left,
            right,
            combinator: multiply,
        })
    }

    pub fn advantage(left: Box<dyn Rollable>, right: Box<dyn Rollable>) -> Box<dyn Rollable> {
        Box::new(Operation {
            left,
            right,
            combinator: advantage,
        })
    }

    pub fn disadvantage(left: Box<dyn Rollable>, right: Box<dyn Rollable>) -> Box<dyn Rollable> {
        Box::new(Operation {
            left,
            right,
            combinator: disadvantage,
        })
    }
}

impl Rollable for Operation {
    fn roll(&self) -> i32 {
        (self.combinator)(&self.left.roll(), &self.right.roll())
    }

    fn plot(&self) -> HashMap<i32, i32> {
        let left = self.left.plot();
        let right = self.right.plot();

        let mut product: HashMap<i32, i32> = HashMap::new();

        left.iter()
            .flat_map(|(left_value, left_count)| {
                right.iter().map(move |(right_value, right_count)| {
                    let value = (self.combinator)(left_value, right_value);
                    (value, left_count * right_count)
                })
            })
            .for_each(|(value, count)| {
                *product.entry(value).or_insert(0) += count;
            });

        product
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use crate::die::Die;

    #[test]
    fn multiply_produces_correct_plot() {
        let operation = Operation::multiply(Box::new(Die::new(4)), Box::new(Die::new(4)));
        let expected: HashMap<i32, i32> = [
            (1, 1),
            (2, 2),
            (3, 2),
            (4, 3),
            (6, 2),
            (8, 2),
            (9, 1),
            (12, 2),
            (16, 1),
        ]
        .iter()
        .cloned()
        .collect();

        let actual = operation.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn two_d6_sum_produces_correct_plot() {
        let operation = Operation::sum(Box::new(Die::new(6)), Box::new(Die::new(6)));
        let expected: HashMap<i32, i32> = [
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 4),
            (6, 5),
            (7, 6),
            (8, 5),
            (9, 4),
            (10, 3),
            (11, 2),
            (12, 1),
        ]
        .iter()
        .cloned()
        .collect();

        let actual = operation.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn advantage_produces_correct_plot() {
        let operation = Operation::advantage(Box::new(Die::new(4)), Box::new(Die::new(4)));
        // 1 1 -> 1
        // 1 2 -> 2
        // 1 3 -> 3
        // 1 4 -> 4
        // 2 1 -> 2
        // 2 2 -> 2
        // 2 3 -> 3
        // 2 4 -> 4
        // 3 1 -> 3
        // 3 2 -> 3
        // 3 3 -> 3
        // 3 4 -> 4
        // 4 1 -> 4
        // 4 2 -> 4
        // 4 3 -> 4
        // 4 4 -> 4
        let expected: HashMap<i32, i32> =
            [(1, 1), (2, 3), (3, 5), (4, 7)].iter().cloned().collect();

        let actual = operation.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn disadvantage_produces_correct_plot() {
        let operation = Operation::disadvantage(Box::new(Die::new(4)), Box::new(Die::new(4)));
        // 1 1 -> 1
        // 1 2 -> 1
        // 1 3 -> 1
        // 1 4 -> 1
        // 2 1 -> 1
        // 2 2 -> 2
        // 2 3 -> 2
        // 2 4 -> 2
        // 3 1 -> 1
        // 3 2 -> 2
        // 3 3 -> 3
        // 3 4 -> 3
        // 4 1 -> 1
        // 4 2 -> 2
        // 4 3 -> 3
        // 4 4 -> 4
        let expected: HashMap<i32, i32> =
            [(1, 7), (2, 5), (3, 3), (4, 1)].iter().cloned().collect();

        let actual = operation.plot();

        assert_eq!(expected, actual);
    }
}
