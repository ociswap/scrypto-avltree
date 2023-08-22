use pretty_assertions::{assert_eq};
use scrypto::prelude::*;

mod helper_avl_tree;

use helper_avl_tree::*;
use radix_engine::{
    system::kernel_modules::execution_trace::{ResourceSpecifier::Amount, ResourceSpecifier::Ids},
};
// use ociswap::math::*;

#[test]
fn test_different_side_balance_left_nr_bf_1(){
    let vector: Vec<i32> = vec![2,1,6,4,7,5];
    let mut to_delete = vec![7,3,2,1,6,4];
    println!("vector: {:?}", vector);
    test_range(vector, to_delete);
}
#[test]
fn test_different_side_balance_left_nr_bf_minus_1(){
    let vector: Vec<i32> = vec![2,1,6,4,7,3];
    let mut to_delete = vec![7,3,2,1,6,4];
    test_range(vector, to_delete);
}

fn test_range(vector: Vec<i32>, to_delete: Vec<i32>) {

    // let to_delete= vec![];
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(true);
    for i in vector.iter() {
        print!("insert: --------------------- {} ", i);
        helper.insert(*i, *i);
        helper.print();
        helper.check_health();
        // value -= 1;
        helper.execute_success(true);
    }
    helper.print();
    helper.check_health();
    helper.execute_success(true);
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
    for i in to_delete.iter().rev() {
        // helper.print();
        helper.delete(*i);
        helper.print();
        println!("Deleting {}", i);
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