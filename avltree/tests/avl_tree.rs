use pretty_assertions::assert_eq;
use radix_engine::system::kernel_modules::execution_trace::{ResourceSpecifier::Amount, ResourceSpecifier::Ids};
use scrypto::prelude::*;

use helper_avl_tree::*;

mod helper_avl_tree;

// structural tests
#[test]
fn test_delete_root() {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    helper.insert(1, 1);
    helper.check_health();
    helper.execute_success(true);
    helper.delete(1);
    helper.check_health();
    helper.execute_success(true);
    let mut minimum = i32::MIN;
    let mut maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("Output: {:?}", output);
    assert_eq!(output.len(), 0, "Something is still present in the tree");
}

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

// Debug tests:
#[test]
fn test_double_insert() {
    let vector: Vec<i32> = (0..20).collect();

    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(false);
    // for i in 1..10{
    // let mut value = 99;

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
fn deletion_with_2_parents_above_but_only_one_balance() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut to_delete = vec![0, 1, 2, 3];
    test_range(vector, to_delete);
}

#[test]
fn deletion_with_replace_direct_below() {
    let vector: Vec<i32> = vec![15, 14, 17, 16];
    let mut to_delete = vec![17];
    test_range(vector, to_delete);
}

#[test]
fn delete_and_balance_at_root() {
    let vector: Vec<i32> = vec![7, 5, 15, 3, 6, 11, 17, 4, 16, 18, 20, 14];
    let mut to_delete = vec![3];
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
fn replace_has_children() {
    let vector = vec![48, 27, 81, 13, 40, 72, 35];
    let mut to_delete = vec![48];
    test_range(vector, to_delete);
}

#[test]
fn replace_has_children_other_direction() {
    let vector = vec![48, 27, 81, 93, 71, 40, 73];
    let mut to_delete = vec![48];
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

#[test]
fn test_more_than_two_balances_in_delete() {
    let vector = vec![25, 20, 30, 10, 23, 26, 33, 31];
    let mut to_delete = vec![25];
    test_range(vector, to_delete);
}

#[test]
fn delete_is_bf_0_but_not_shorten() {
    let vector = vec![44, 39, 49, 36, 42, 46, 51, 34, 40, 43, 47, 52, 41];
    let mut to_delete = vec![36];
    test_range(vector, to_delete);
}

// #[test]
fn test_random2() {
    // let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6, 59, 72, 46, 46, 8, 81, 93, 64, 98, 11, 92, 10, 26, 34, 20, 13, 0, 42, 70, 87, 94, 2, 60, 14, 39, 18, 77, 41, 56, 15, 75, 79, 57, 33, 32, 21, 83, 100, 31, 9, 66, 88, 63, 30, 19, 37, 17, 28, 51, 67, 53, 4, 24, 44, 95, 38, 52, 71, 29, 36, 89, 3, 73, 84, 80, 43, 55, 91, 50, 76, 22, 49, 86, 47, 23, 7, 58, 54, 16, 25, 12, 68, 61, 96, 97, 65, 78, 45, 85, 69, 62];
    let vector = vec![14, 53, 63, 96, 66, 74, 12, 48, 87, 60, 59, 67, 58, 75, 76, 23, 38, 16, 79, 32, 27, 37, 88, 78, 50, 26, 45, 93, 22, 35, 36, 98, 46, 18, 57, 81, 82, 30, 73, 61, 34, 83, 77, 84, 25, 69, 29, 89, 95, 19, 13, 33, 97, 55, 49, 51, 20, 21, 42, 54, 94, 90, 62, 43, 15, 40, 71, 86, 92, 99, 64, 39, 28, 70, 72, 24, 65, 44, 47, 56, 91, 11, 52, 68, 17, 41, 31, 80, 85, 10];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}

// #[test]
fn test_random3() {
    let vector = vec![14, 60, 59, 95, 20, 42, 86, 39, 57, 98, 74, 34, 68, 29, 91, 92, 36, 56, 66, 50, 62, 58, 11, 37, 15, 52, 38, 17, 12, 79, 89, 53, 16, 65, 25, 64, 30, 97, 23, 24, 87, 10, 94, 44, 67, 76, 47, 61, 75, 81, 70, 26, 71, 72, 54, 35, 41, 27, 51, 84, 73, 55, 63, 13, 93, 31, 82, 69, 96, 80, 45, 49, 18, 32, 33, 28, 40, 99, 22, 43, 21, 77, 83, 90, 88, 19, 48, 78, 46, 85];
    let to_delete = vector.clone();
    test_range(vector, to_delete);
}

#[test]
fn replace_with_left_child_different_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 72, 75, 73];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

#[test]
fn replace_with_left_child_same_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 73, 75, 71];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

#[test]
fn replace_with_right_child_same_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 73, 75, 76];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

#[test]
fn replace_with_right_childdifferent_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 73, 76, 75];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

// #[test]
fn test_random() {
    let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6, 59, 72, 46, 46, 8, 81, 93, 64, 98, 11, 92, 10, 26, 34, 20, 13, 0, 42, 70, 87, 94, 2, 60, 14, 39, 18, 77, 41, 56, 15, 75, 79, 57, 33, 32, 21, 83, 100, 31, 9, 66, 88, 63, 30, 19, 37, 17, 28, 51, 67, 53, 4, 24, 44, 95, 38, 52, 71, 29, 36, 89, 3, 73, 84, 80, 43, 55, 91, 50, 76, 22, 49, 86, 47, 23, 7, 58, 54, 16, 25, 12, 68, 61, 96, 97, 65, 78, 45, 85, 69, 62];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}

#[test]
fn remove_last_inserted() {
    let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6];
    let mut to_delete = vec![6];
    test_range(vector, to_delete);
}

#[test]
fn replace_2_layers_above() {
    // rewiring of node in the middle could go wrong because it has to be in memory.
    let vector: Vec<i32> = vec![18, 15, 21, 12, 16, 20];
    let mut to_delete = vec![18];
    test_range(vector, to_delete);
}

#[test]
fn replace_node_has_not_bf_0_after_rewire() {
    let vector: Vec<i32> = vec![18, 15, 21, 12, 16, 20];
    let mut to_delete = vec![16];
    test_range(vector, to_delete);
}

#[test]
fn test_balancing_of_subtree_with_different_directions() {
    // Test balancing of subtree with different direction.
    let vector = vec![10, 25, 11, 13, 15, 12, 18, 20, 21, 22];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}

fn test_range(vector: Vec<i32>, to_delete: Vec<i32>) {

    // let to_delete= vec![];
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        // helper.print();
        helper.check_health();
        // value -= 1;
        helper.execute_success(true);
    }

    let mut minimum = i32::MIN;
    let mut maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("to_delete: {:?}", to_delete);
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    for i in vector.iter() {
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
    // helper.print();
    // helper.check_health();
    // helper.execute_success(true);
    for i in to_delete.iter().rev() {
        // helper.print();
        helper.delete(*i);
        // helper.print();
        // println!("Deleting {}", i);
        helper.check_health();
        helper.execute_success(true);
    }
    // helper.print();
    let mut minimum = i32::MAX;
    let mut maximum = i32::MIN;
    for i in vector.iter() {
        if i < &minimum {
            minimum = *i;
        }
        if i > &maximum {
            maximum = *i;
        }
    }
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("to_delete: {:?}", to_delete);
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    for i in vector.iter() {
        assert!(output.contains(&i) || to_delete.contains(&i), "i not contained in the tree {}", i);
    }
    for i in output.iter() {
        assert!(vector.contains(&i) && !to_delete.contains(&i), "elements in the tree that should be deleted or present {}", i);
    }
}
