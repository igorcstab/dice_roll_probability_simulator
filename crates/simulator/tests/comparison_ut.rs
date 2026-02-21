use simulator::comparison::{Operation, compare};

#[test]
fn greater_or_equal_returns_true_when_greater() {
    let result = compare(8, Operation::GreaterOrEqual, 7);
    assert!(result);
}

#[test]
fn greater_or_equal_returns_true_when_equal() {
    let result = compare(7, Operation::GreaterOrEqual, 7);
    assert!(result);
}

#[test]
fn greater_or_equal_returns_false_when_smaller() {
    let result = compare(6, Operation::GreaterOrEqual, 7);
    assert!(!result);
}

#[test]
fn greater_returns_true_when_greater() {
    let result = compare(8, Operation::Greater, 7);
    assert!(result);
}

#[test]
fn greater_returns_false_when_equal() {
    let result = compare(7, Operation::Greater, 7);
    assert!(!result);
}

#[test]
fn greater_returns_false_when_smaller() {
    let result = compare(6, Operation::Greater, 7);
    assert!(!result);
}

#[test]
fn smaller_or_equal_returns_false_when_greater() {
    let result = compare(8, Operation::SmallerOrEqual, 7);
    assert!(!result);
}

#[test]
fn smaller_or_equal_returns_true_when_equal() {
    let result = compare(7, Operation::SmallerOrEqual, 7);
    assert!(result);
}

#[test]
fn smaller_or_equal_returns_true_when_smaller() {
    let result = compare(6, Operation::SmallerOrEqual, 7);
    assert!(result);
}

#[test]
fn smaller_returns_false_when_greater() {
    let result = compare(8, Operation::Smaller, 7);
    assert!(!result);
}

#[test]
fn smaller_returns_false_when_equal() {
    let result = compare(7, Operation::Smaller, 7);
    assert!(!result);
}

#[test]
fn smaller_returns_true_when_smaller() {
    let result = compare(6, Operation::Smaller, 7);
    assert!(result);
}

#[test]
fn equal_returns_true_when_equal() {
    let result = compare(7, Operation::Equal, 7);
    assert!(result);
}

#[test]
fn equal_returns_false_when_greater() {
    let result = compare(8, Operation::Equal, 7);
    assert!(!result);
}

#[test]
fn equal_returns_false_when_smaller() {
    let result = compare(6, Operation::Equal, 7);
    assert!(!result);
}
