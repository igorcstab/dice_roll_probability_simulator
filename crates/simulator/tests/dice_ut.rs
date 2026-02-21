use simulator::dice::eval_dice_range;

#[test]
fn two_d4_returns_2_to_8() {
    let result = eval_dice_range(2, 4);
    let expected = vec![2, 3, 4, 5, 6, 7, 8];
    assert_eq!(result, expected);
}

#[test]
fn one_d6_returns_1_to_6() {
    let result = eval_dice_range(1, 6);
    let expected = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(result, expected);
}

#[test]
fn two_d6_returns_2_to_12() {
    let result = eval_dice_range(2, 6);
    let expected = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    assert_eq!(result, expected);
}

#[test]
fn two_d8_returns_2_to_16() {
    let result = eval_dice_range(2, 8);
    let expected = vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    assert_eq!(result, expected);
}

#[test]
fn two_d10_returns_2_to_20() {
    let result = eval_dice_range(2, 10);
    let expected = vec![
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    assert_eq!(result, expected);
}

#[test]
fn two_d12_returns_2_to_24() {
    let result = eval_dice_range(2, 12);
    let expected = vec![
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ];
    assert_eq!(result, expected);
}

#[test]
fn two_d20_returns_2_to_40() {
    let result = eval_dice_range(2, 20);
    let expected = vec![
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
        27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    ];
    assert_eq!(result, expected);
}

#[test]
fn zero_dice_returns_empty_vector() {
    let result = eval_dice_range(0, 6);
    let expected = vec![];
    assert_eq!(result, expected);
}

#[test]
fn faceless_dice_returns_empty_vector() {
    let result = eval_dice_range(6, 0);
    let expected = vec![];
    assert_eq!(result, expected);
}

#[test]
fn nothing_selected_returns_empty_vector() {
    let result = eval_dice_range(0, 0);
    let expected = vec![];
    assert_eq!(result, expected);
}
