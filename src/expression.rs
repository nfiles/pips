use rand::Rng;
use std::collections::HashMap;

use crate::operators::{advantage, compare, disadvantage, multiply, sum, BinaryOperator};

#[derive(Clone)]
pub enum Expression<'a> {
    Die(i32),
    Constant(i32),

    Sum(&'a Expression<'a>, &'a Expression<'a>),
    Multiply(&'a Expression<'a>, &'a Expression<'a>),
    Advantage(&'a Expression<'a>),
    Disadvantage(&'a Expression<'a>),
    Compare(&'a Expression<'a>, &'a Expression<'a>),
}

use Expression::*;

impl<'a> Expression<'a> {
    pub fn roll(&self) -> i32 {
        match self {
            Constant(num) => *num,
            Die(num) => rand::thread_rng().gen_range(1, num + 1),

            Sum(left, right) => left.roll() + right.roll(),
            Multiply(left, right) => left.roll() * right.roll(),
            Advantage(expr) => {
                let left = expr.roll();
                let right = expr.roll();
                if left > right {
                    left
                } else {
                    right
                }
            }
            Disadvantage(expr) => {
                let left = expr.roll();
                let right = expr.roll();
                if left < right {
                    left
                } else {
                    right
                }
            }
            Compare(left, right) => {
                let left = left.roll();
                let right = right.roll();
                if left > right {
                    1
                } else if left == right {
                    0
                } else {
                    -1
                }
            }
        }
    }

    pub fn plot(&self) -> HashMap<i32, i32> {
        // get the root cases out of the way
        if let Constant(num) = self {
            return [(*num, 1)].iter().cloned().collect();
        }
        if let Die(num) = self {
            return (1..num + 1).map(|i| (i, 1)).collect();
        }

        // handle the more complicated expressions
        let (operator, left, right): (BinaryOperator, _, _) = match self {
            Sum(left, right) => (sum, left, right),
            Multiply(left, right) => (multiply, left, right),
            Advantage(expr) => (advantage, expr, expr),
            Disadvantage(expr) => (disadvantage, expr, expr),
            Compare(left, right) => (compare, left, right),

            Constant(_) => panic!("this should never happen"),
            Die(_) => panic!("this should never happen"),
        };

        let left = left.plot();
        let right = right.plot();

        let mut product: HashMap<i32, i32> = HashMap::new();

        left.iter()
            .flat_map(|(left_value, left_count)| {
                right.iter().map(move |(right_value, right_count)| {
                    let value = (operator)(left_value, right_value);
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

    #[test]
    fn multiply_produces_correct_plot() {
        let operation = Expression::Multiply(&Expression::Die(4), &Expression::Die(4));
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
        let operation = Expression::Sum(&Expression::Die(6), &Expression::Die(6));
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
        let operation = Expression::Advantage(&Expression::Die(4));
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
        let operation = Expression::Disadvantage(&Expression::Die(4));
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

    #[test]
    fn contest_produces_correct_plot() {
        let operation = Expression::Compare(&Expression::Die(2), &Expression::Die(3));

        // 1 1 -> 0
        // 1 2 -> -1
        // 1 3 -> -1
        // 2 1 -> 1
        // 2 2 -> 0
        // 2 3 -> -1
        let expected: HashMap<i32, i32> = [(-1, 3), (0, 2), (1, 1)].iter().cloned().collect();

        let actual = operation.plot();

        assert_eq!(expected, actual);
    }
}
