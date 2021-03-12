use crate::traits::RollResult;

/// An operator that performs some logic on two inputs
pub type BinaryOperator = fn(left: &RollResult, right: &RollResult) -> RollResult;

/// Find the sum of two rolls
pub fn sum(left: &RollResult, right: &RollResult) -> RollResult {
    (*left) + (*right)
}

/// Find the difference between two rolls
pub fn difference(left: &RollResult, right: &RollResult) -> RollResult {
    (*left) - (*right)
}

/// Multiple two rolls
pub fn multiply(left: &RollResult, right: &RollResult) -> RollResult {
    (*left) * (*right)
}

/// Divide two rolls
pub fn divide(left: &RollResult, right: &RollResult) -> RollResult {
    // TODO: account for divide-by-zero
    (*left) / (*right)
}

/// Take the greater of two rolls
pub fn advantage(left: &RollResult, right: &RollResult) -> RollResult {
    if *left > *right {
        *left
    } else {
        *right
    }
}

/// Take the lesser of two rolls
pub fn disadvantage(left: &RollResult, right: &RollResult) -> RollResult {
    if *left < *right {
        *left
    } else {
        *right
    }
}

/// Compare two rolls
///
/// returns:
/// - `1` if left is greater than right
/// - `0` if left is equal to right
/// - `-1` if left is less than right
pub fn compare(left: &RollResult, right: &RollResult) -> RollResult {
    if *left > *right {
        1
    } else if *left == *right {
        0
    } else {
        -1
    }
}

/// 1 if right is greater than left else 0
pub fn greater_than(left: &RollResult, right: &RollResult) -> RollResult {
    if *left > *right {
        1
    } else {
        0
    }
}

/// 1 if right is greater than or equal to left else 0
pub fn greater_than_or_equal_to(left: &RollResult, right: &RollResult) -> RollResult {
    if *left >= *right {
        1
    } else {
        0
    }
}

/// 1 if right is less than left else 0
pub fn less_than(left: &RollResult, right: &RollResult) -> RollResult {
    if *left > *right {
        1
    } else {
        0
    }
}

/// 1 if right is less than left else 0
pub fn less_than_or_equal_to(left: &RollResult, right: &RollResult) -> RollResult {
    if *left >= *right {
        1
    } else {
        0
    }
}

/// 1 if right is equal to left else 0
pub fn equal_to(left: &RollResult, right: &RollResult) -> RollResult {
    if *left == *right {
        1
    } else {
        0
    }
}
