//! Expression Module

use rand::Rng;
use std::collections::HashMap;

use crate::operators::{advantage, compare, disadvantage, multiply, sum, BinaryOperator};
use crate::traits::Rollable;

/// Represents a dice roll expression
#[derive(Clone, Debug, PartialEq)]
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
    /// retrieve the operation encapsulated by the given `Expression`,
    /// represented by a binary operator and left/right expressions
    fn get_operation(&self) -> Option<(BinaryOperator, &Expression, &Expression)> {
        match self {
            Constant(_) => None,
            Die(_) => None,

            Sum(left, right) => Some((sum, left, right)),
            Multiply(left, right) => Some((multiply, left, right)),
            Advantage(expr) => Some((advantage, expr, expr)),
            Disadvantage(expr) => Some((disadvantage, expr, expr)),
            Compare(left, right) => Some((compare, left, right)),
        }
    }
}

impl<'a> Rollable for Expression<'a> {
    /// Get a single value from the roll expression
    fn roll(&self) -> i32 {
        // get the root cases out of the way
        if let Constant(num) = self {
            return *num;
        }
        if let Die(max) = self {
            return rand::thread_rng().gen_range(1, max + 1);
        }

        let (operator, left, right) = self
            .get_operation()
            .expect("expression does not represent an operation");

        (operator)(&left.roll(), &right.roll())
    }

    /// Create a list of all possible outcomes and their possibility
    fn plot(&self) -> HashMap<i32, i32> {
        // get the root cases out of the way
        if let Constant(num) = self {
            return [(*num, 1)].iter().cloned().collect();
        }
        if let Die(num) = self {
            return (1..num + 1).map(|i| (i, 1)).collect();
        }

        // handle the more complicated expressions
        let (operator, left, right) = self
            .get_operation()
            .expect("expression does not represent an operation");

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
        let expression = Expression::Multiply(&Expression::Die(4), &Expression::Die(4));
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

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn sum_produces_correct_plot() {
        let expression = Expression::Sum(&Expression::Die(6), &Expression::Die(6));
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

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn advantage_produces_correct_plot() {
        let expression = Expression::Advantage(&Expression::Die(4));
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

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn disadvantage_produces_correct_plot() {
        let expression = Expression::Disadvantage(&Expression::Die(4));
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

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn contest_produces_correct_plot() {
        let expression = Expression::Compare(&Expression::Die(2), &Expression::Die(3));

        // 1 1 -> 0
        // 1 2 -> -1
        // 1 3 -> -1
        // 2 1 -> 1
        // 2 2 -> 0
        // 2 3 -> -1
        let expected: HashMap<i32, i32> = [(-1, 3), (0, 2), (1, 1)].iter().cloned().collect();

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn compare_produces_correct_plot() {
        let expression = Expression::Compare(&Expression::Die(3), &Expression::Constant(2));

        // 1 2 -> -1
        // 2 2 -> 0
        // 3 2 -> 1
        let expected: HashMap<i32, i32> = [(-1, 1), (0, 1), (1, 1)].iter().cloned().collect();

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }
}
