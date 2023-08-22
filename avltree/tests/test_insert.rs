use pretty_assertions::assert_eq;
use radix_engine::system::kernel_modules::execution_trace::{ResourceSpecifier::Amount, ResourceSpecifier::Ids};
use scrypto::prelude::*;

mod test_utils;
use crate::test_utils::test_range;
mod helper_avl_tree;
use helper_avl_tree::*;

// structural tests
#[test]
fn test_same_side_balance_left() {
    let vector: Vec<i32> = vec![1, 2, 3];
    let mut to_delete = vec![3, 2, 1];
    test_range(vector, to_delete);
}

#[test]
fn test_same_side_balance_right() {
    let vector: Vec<i32> = vec![3, 2, 1];
    let mut to_delete = vec![3, 2, 1];
    test_range(vector, to_delete);
}

#[test]
fn test_balance_wtih_child_zero_bf_right() {
    let vector: Vec<i32> = vec![2, 1, 4];
    let mut to_delete = vec![3, 2, 1];
    test_range(vector, to_delete);
}

#[test]
fn test_different_side_balance_right() {
    let vector: Vec<i32> = vec![1, 3, 2];
    let mut to_delete = vec![3, 2, 1];
    test_range(vector, to_delete);
}

#[test]
fn test_different_side_balance_left() {
    let vector: Vec<i32> = vec![3, 1, 2];
    let mut to_delete = vec![1, 2, 3];
    test_range(vector, to_delete);
}

#[test]
fn test_different_side_balance_left_nr_bf_1() {
    let vector: Vec<i32> = vec![2, 1, 6, 4, 7, 5];
    let mut to_delete = vec![7, 3, 2, 1, 6, 4];
    test_range(vector, to_delete);
}

#[test]
fn test_different_side_balance_left_nr_bf_minus_1() {
    let vector: Vec<i32> = vec![2, 1, 6, 4, 7, 3];
    let mut to_delete = vec![7, 3, 2, 1, 6, 4];
    test_range(vector, to_delete);
}

#[test]
fn test_double_insert() {
    let vector: Vec<i32> = (0..20).collect();

    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);

    for i in vector.iter() {
        helper.insert(*i, 1);
        helper.execute_success(true);
    }
    for i in vector.iter() {
        helper.insert(*i, 0);
        helper.check_health();
        helper.execute_success(true);
    }
    let mut minimum = i32::MIN;
    let mut maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    for value in output[0].clone() {
        assert_eq!(value, 0, "value should have been overwritten with 0");
    }
}
