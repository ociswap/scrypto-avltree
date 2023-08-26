use pretty_assertions::assert_eq;
use scrypto::prelude::*;

mod helper_avl_tree;
use helper_avl_tree::*;

#[test]
fn check_that_range_is_sorted(){
    let mut helper = TestHelper::new();

    helper.instantiate_default(true);
    let vector = vec![13,24,43,23,12,23,13,42,53,54,21,11,12,14,16];
    // let to_delete= vec![];
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        helper.check_health();
        helper.execute_success(true);
    }

    let mut minimum = i32::MIN;
    let mut maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last= i32::MIN;
    for i in output.iter() {
        assert!(last < *i, "range_not_sorted last {}, i {},  {:?}", last, i, vector);
        last = *i;
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
    helper.get_range_mut(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last= i32::MIN;
    for i in output.iter() {
        assert!(last < *i, "range_not_sorted last {}, i {},  {:?}", last, i, vector);
        last = *i;
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
}

#[test]
fn check_that_range_only_contains_range(){
    let mut helper = TestHelper::new();
    helper.instantiate_default(true);
    let vector: Vec<i32> = (10..30).collect();
    // let to_delete= vec![];
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        helper.check_health();
        helper.execute_success(true);
    }

    let mut minimum = 15;
    let mut maximum = 25;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last= i32::MIN;
    for i in output.clone().into_iter() {
        assert!(last < i, "range_not_sorted {:?}", vector.clone());
        last = i;
        assert!(vector.contains(&i), "i not contained in the tree {}", i);
        assert!(i >= 15, "All elements should be bigger 15");
        assert!(i <= 25, "All elements should be bigger 25");
    }
    helper.get_range_mut(minimum, maximum);
    let receipt = helper.execute_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last= i32::MIN;
    for i in output.clone().into_iter() {
        assert!(last < i, "range_not_sorted {:?}", vector.clone());
        last = i;
        assert!(vector.contains(&i), "i not contained in the tree {}", i);
        assert!(i >= 15, "All elements should be bigger 15");
        assert!(i <= 25, "All elements should be bigger 25");
    }
}