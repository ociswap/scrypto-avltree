use pretty_assertions::assert_eq;
use scrypto::prelude::*;
mod helper_avl_tree;
use helper_avl_tree::*;

// Debug tests:
#[test]
fn test_increasing() {
    let vector: Vec<i32> = (0..20).collect();
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}


#[test]
fn test_decrease() {
    let vector: Vec<i32> = (0..20).rev().collect();
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}


#[test]
fn replace_has_children_other_direction() {
    let vector = vec![48, 27, 81, 93, 71, 40, 73];
    let to_delete = vec![48];
    test_range(vector, to_delete);
}

#[test]
fn replace_has_to_change_balance() {
    let vector = vec![74, 11, 48, 27, 90, 35, 82, 48, 10, 12, 59, 72, 46, 40, 13, 81, 93, 21, 30, 37, 23, 42, 29, 22, 98, 31, 31, 14, 73];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}


