//! Expression Module

use crate::traits::PlotResult;
use crate::traits::PlotTable;
use crate::traits::RollResult;
use rand::Rng;
use std::collections::HashMap;

use crate::operators::{
    advantage, difference, disadvantage, divide, equal_to, greater_than, greater_than_or_equal_to,
    less_than, less_than_or_equal_to, multiply, sum, BinaryOperator,
};
use crate::traits::Rollable;

#[derive(Clone, Debug, PartialEq)]
pub enum Comparison {
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    EqualTo,
}

use Comparison::*;

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
    Compare(Box<Expression>, Box<Expression>, Comparison),
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
            Compare(left, right, comparison) => {
                let compare = match comparison {
                    GreaterThan => greater_than,
                    GreaterThanOrEqualTo => greater_than_or_equal_to,
                    LessThan => less_than,
                    LessThanOrEqualTo => less_than_or_equal_to,
                    EqualTo => equal_to,
                };
                Some((compare, left, right))
            }
        }
    }
}

impl Rollable for Expression {
    /// Get a single value from the roll expression
    fn roll(&self) -> RollResult {
        // get the root cases out of the way
        if let Constant(num) = self {
            return *num;
        }
        if let Die(max) = self {
            return rand::thread_rng().gen_range(1, *max as RollResult + 1);
        }

        let (operator, left, right) = self
            .get_operation()
            .expect("expression does not represent an operation");

        (operator)(&left.roll(), &right.roll())
    }

    /// Create a list of all possible outcomes and their possibility
    fn plot(&self) -> PlotResult {
        // get the root cases out of the way
        if let Constant(num) = self {
            return PlotResult {
                total: 1.0,
                plot: [(*num, 1.0)].iter().cloned().collect(),
            };
        }
        if let Die(num) = self {
            let total = *num as f32;
            return PlotResult {
                total,
                plot: (1..num + 1)
                    .map(|i| (i as RollResult, 1.0 / total))
                    .collect(),
            };
        }

        // handle the more complicated expressions
        let (operator, left, right) = self
            .get_operation()
            .expect("expression does not represent an operation");

        let left = left.plot();
        let right = right.plot();

        let mut product: PlotTable = HashMap::new();

        left.plot
            .iter()
            .flat_map(|(left_value, left_chance)| {
                right.plot.iter().map(move |(right_value, right_chance)| {
                    let value = (operator)(left_value, right_value);
                    (value, left_chance * right_chance)
                })
            })
            .for_each(|(value, count)| {
                *product.entry(value).or_insert(0.0) += count;
            });

        PlotResult {
            total: left.total * right.total,
            plot: product,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl PlotResult {
        /// de-normalize the table of possible outcomes
        pub fn simplify(&self) -> HashMap<i32, i32> {
            self.plot
                .iter()
                .map(|(value, chance)| {
                    let outcomes = (chance * self.total) as i32;
                    (*value, outcomes)
                })
                .collect()
        }
    }

    #[test]
    fn simplify_produces_correct_table() {
        let plot_result = PlotResult {
            total: 10.0,
            plot: [(1, 0.1), (2, 0.2), (3, 0.3), (4, 0.4)]
                .iter()
                .cloned()
                .collect(),
        };
        let expected: HashMap<i32, i32> =
            [(1, 1), (2, 2), (3, 3), (4, 4)].iter().cloned().collect();

        let actual = plot_result.simplify();

        assert_eq!(expected, actual);
    }

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

        let actual = expression.plot().simplify();

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

        let actual = expression.plot().simplify();

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

        let actual = expression.plot().simplify();

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

        let actual = expression.plot().simplify();

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

        let actual = expression.plot().simplify();

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

        let actual = expression.plot().simplify();

        assert_eq!(expected, actual);
    }

    #[test]
    #[ignore = "not implemented"]
    fn contest_produces_correct_plot() {
        let expression = Expression::Compare(
            Box::new(Expression::Die(2)),
            Box::new(Expression::Die(3)),
            Comparison::GreaterThan,
        );

        // 1 1 -> 0
        // 1 2 -> 0
        // 1 3 -> 0
        // 2 1 -> 1
        // 2 2 -> 0
        // 2 3 -> 0
        let expected: HashMap<i32, i32> = [(-1, 3), (0, 2), (1, 1)].iter().cloned().collect();

        let actual = expression.plot().simplify();

        assert_eq!(expected, actual);
    }

    #[test]
    fn compare_produces_correct_plot() {
        let left = Expression::Die(3);
        let right = Expression::Die(3);
        // 1 1
        // 1 2
        // 1 3
        // 2 1
        // 2 2
        // 2 3
        // 3 1
        // 3 2
        // 3 3
        let cases: Vec<(Comparison, &[(i32, i32)])> = vec![
            (Comparison::GreaterThan, &[(0, 6), (1, 3)]),
            (Comparison::GreaterThanOrEqualTo, &[(0, 3), (1, 6)]),
            (Comparison::LessThan, &[(0, 6), (1, 3)]),
            (Comparison::LessThanOrEqualTo, &[(0, 3), (1, 6)]),
            (Comparison::EqualTo, &[(0, 6), (1, 3)]),
        ];

        for (comparison, options) in cases {
            let expression = Expression::Compare(
                Box::new(left.clone()),
                Box::new(right.clone()),
                comparison.clone(),
            );

            let expected: HashMap<i32, i32> = options.iter().cloned().collect();
            let actual = expression.plot().simplify();

            assert_eq!(expected, actual, "{:?}", comparison);
        }
    }
}
