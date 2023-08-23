use pretty_assertions::assert_eq;
use radix_engine::system::kernel_modules::execution_trace::{ResourceSpecifier::Amount, ResourceSpecifier::Ids};
use scrypto::prelude::*;
use crate::helper_avl_tree::TestHelper;


pub fn test_range(vector: Vec<i32>, to_delete: Vec<i32>) {
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        // helper.print();
        helper.check_health();
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
    let mut last= i32::MIN;
    for i in output.clone(){
        assert!(last < i, "range_not_sorted {:?}", vector);
        last = i;
    }
    for i in vector.clone() {
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
    for i in to_delete.iter().rev() {
        helper.delete(*i);
        // helper.print();
        println!("Deleting {}", i);
        helper.check_health();
        helper.execute_success(true);
    }
    // helper.print();
    let mut minimum = i32::MAX;
    let mut maximum = i32::MIN;
    for i in vector.clone() {
        if i < minimum {
            minimum = i;
        }
        if i > maximum {
            maximum = i;
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
