//! Expression Module

use rand::Rng;
use std::collections::HashMap;

use crate::operators::{
    advantage, compare, difference, disadvantage, divide, multiply, sum, BinaryOperator,
};
use crate::traits::Rollable;

/// Represents a dice roll expression
#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Die(u32),
    Constant(i32),

    Sum(Box<Expression>, Box<Expression>),
    Diff(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Advantage(Box<Expression>),
    Disadvantage(Box<Expression>),
    Compare(Box<Expression>, Box<Expression>),
}

use Expression::*;

impl Expression {
    /// retrieve the operation encapsulated by the given `Expression`,
    /// represented by a binary operator and left/right expressions
    fn get_operation(&self) -> Option<(BinaryOperator, &Box<Expression>, &Box<Expression>)> {
        match self {
            Constant(_) => None,
            Die(_) => None,

            Sum(left, right) => Some((sum, left, right)),
            Diff(left, right) => Some((difference, left, right)),
            Multiply(left, right) => Some((multiply, left, right)),
            Divide(left, right) => Some((divide, left, right)),
            Advantage(expr) => Some((advantage, expr, expr)),
            Disadvantage(expr) => Some((disadvantage, expr, expr)),
            Compare(left, right) => Some((compare, left, right)),
        }
    }
}

impl Rollable for Expression {
    /// Get a single value from the roll expression
    fn roll(&self) -> i32 {
        // get the root cases out of the way
        if let Constant(num) = self {
            return *num;
        }
        if let Die(max) = self {
            return rand::thread_rng().gen_range(1, *max as i32 + 1);
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
            return (1..num + 1).map(|i| (i as i32, 1)).collect();
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
        let expression =
            Expression::Multiply(Box::new(Expression::Die(4)), Box::new(Expression::Die(4)));
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
    fn divide_produces_correct_plot() {
        let expression =
            Expression::Divide(Box::new(Expression::Die(4)), Box::new(Expression::Die(4)));
        // 1 1 -> 1
        // 1 2 -> 0
        // 1 3 -> 0
        // 1 4 -> 0
        // 2 1 -> 2
        // 2 2 -> 1
        // 2 3 -> 0
        // 2 4 -> 0
        // 3 1 -> 3
        // 3 2 -> 1
        // 3 3 -> 1
        // 3 4 -> 0
        // 4 1 -> 4
        // 4 2 -> 2
        // 4 3 -> 1
        // 4 4 -> 1
        let expected: HashMap<i32, i32> = [(0, 6), (1, 6), (2, 2), (3, 1), (4, 1)]
            .iter()
            .cloned()
            .collect();

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn sum_produces_correct_plot() {
        let expression =
            Expression::Sum(Box::new(Expression::Die(6)), Box::new(Expression::Die(6)));
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
    fn difference_produces_correct_plot() {
        let expression =
            Expression::Diff(Box::new(Expression::Die(4)), Box::new(Expression::Die(4)));
        // 1 1 -> 0
        // 1 2 -> -1
        // 1 3 -> -2
        // 1 4 -> -3
        // 2 1 -> 1
        // 2 2 -> 0
        // 2 3 -> -1
        // 2 4 -> -2
        // 3 1 -> 2
        // 3 2 -> 1
        // 3 3 -> 0
        // 3 4 -> -1
        // 4 1 -> 3
        // 4 2 -> 2
        // 4 3 -> 1
        // 4 4 -> 0
        let expected: HashMap<i32, i32> =
            [(-3, 1), (-2, 2), (-1, 3), (0, 4), (1, 3), (2, 2), (3, 1)]
                .iter()
                .cloned()
                .collect();

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }

    #[test]
    fn advantage_produces_correct_plot() {
        let expression = Expression::Advantage(Box::new(Expression::Die(4)));
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
        let expression = Expression::Disadvantage(Box::new(Expression::Die(4)));
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
        let expression =
            Expression::Compare(Box::new(Expression::Die(2)), Box::new(Expression::Die(3)));

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
        let expression = Expression::Compare(
            Box::new(Expression::Die(3)),
            Box::new(Expression::Constant(2)),
        );

        // 1 2 -> -1
        // 2 2 -> 0
        // 3 2 -> 1
        let expected: HashMap<i32, i32> = [(-1, 1), (0, 1), (1, 1)].iter().cloned().collect();

        let actual = expression.plot();

        assert_eq!(expected, actual);
    }
}
