use scrypto::prelude::*;
use scrypto_testenv::TestHelperExecution;

use helper_avl_tree::*;

mod helper_avl_tree;

#[test]
fn range_out_of_bounds() {
    let mut helper = TestHelper::new();
    helper.instantiate_default(true);
    for i in 1..5 {
        helper.insert(i, i);
    }
    helper.check_health();
    helper.execute_expect_success(true);
    let minimum = 6;
    let maximum = 10;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    assert_eq!(output.len(), 0);
    assert!(!output.contains(&5));
}

#[test]
fn start_included_end_excluded() {
    let mut helper = TestHelper::new();
    helper.instantiate_default(true);
    for i in 1..5 {
        helper.insert(i, i);
    }
    helper.check_health();
    helper.execute_expect_success(true);
    let minimum = 1;
    let maximum = 5;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    let expected = vec![1, 2, 3, 4];
    for i in 0..output.len() {
        assert_eq!(output[i], expected[i]);
    }
    assert!(!output.contains(&5));
    println!("output {:?}", output);
    println!("expected {:?}", expected);
    assert_eq!(output.len(), expected.len());
}

#[test]
fn check_that_back_range_is_sorted() {
    let mut helper = TestHelper::new();

    helper.instantiate_default(true);
    let vector = vec![13, 24, 43, 23, 12, 23, 13, 42, 53, 54, 21, 11, 12, 14, 16];
    // let to_delete= vec![];
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        helper.check_health();
        helper.execute_expect_success(true);
    }

    let minimum = i32::MIN;
    let maximum = i32::MAX;
    helper.get_range_back(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range_back");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last = i32::MAX;
    for i in output.iter() {
        assert!(last > *i, "range_not_sorted last {}, i {},  {:?}", last, i, vector);
        last = *i;
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
}

#[test]
fn check_that_range_is_sorted() {
    let mut helper = TestHelper::new();

    helper.instantiate_default(true);
    let vector = vec![13, 24, 43, 23, 12, 23, 13, 42, 53, 54, 21, 11, 12, 14, 16];
    // let to_delete= vec![];
    let mut helper = TestHelper::new();
    // Print the shuffled vector
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        helper.check_health();
        helper.execute_expect_success(true);
    }

    let minimum = i32::MIN;
    let maximum = i32::MAX;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last = i32::MIN;
    for i in output.iter() {
        assert!(last < *i, "range_not_sorted last {}, i {},  {:?}", last, i, vector);
        last = *i;
        assert!(output.contains(&i), "i not contained in the tree {}", i);
    }
}

#[test]
fn check_that_range_back_only_contains_range() {
    let mut helper = TestHelper::new();
    helper.instantiate_default(true);
    let vector: Vec<i32> = (10..30).collect();
    let mut helper = TestHelper::new();
    helper.instantiate_default(false);
    for i in vector.iter() {
        helper.insert(*i, *i);
        helper.check_health();
        helper.execute_expect_success(true);
    }

    let minimum = 15;
    let maximum = 25;
    helper.get_range_back_both_excluded(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range_back_both_excluded");
    let output = output[0].clone();
    let mut last = i32::MAX;
    println!("output: {:?}", output);
    assert!(!output.contains(&15));
    assert!(!output.contains(&25));
    for i in output.clone().into_iter() {
        assert!(last > i, "range_not_sorted {:?}", output.clone());
        last = i;
        assert!(vector.contains(&i), "i not contained in the tree {}", i);
        assert!(i > 15, "All elements should be bigger 15");
        assert!(i < 25, "All elements should be bigger 25");
    }
    helper.get_range_back_both_included(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range_back_both_included");
    let output = output[0].clone();
    let mut last = i32::MAX;
    assert!(output.contains(&15));
    assert!(output.contains(&25));
    for i in output.clone().into_iter() {
        assert!(last > i, "range_not_sorted {:?}", output.clone());
        last = i;
        assert!(vector.contains(&i), "i not contained in the tree {}", i);
        assert!(i >= 15, "All elements should be bigger 15");
        assert!(i <= 25, "All elements should be bigger 25");
    }
    helper.get_range_back(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range_back");
    let output = output[0].clone();
    let mut last = i32::MAX;
    assert!(output.contains(&15));
    assert!(!output.contains(&25));
    for i in output.clone().into_iter() {
        assert!(last > i, "range_not_sorted {:?}", vector.clone());
        last = i;
        assert!(vector.contains(&i), "i not contained in the tree {}", i);
        assert!(i >= 15, "All elements should be bigger 15");
        assert!(i < 25, "All elements should be bigger 25");
    }
}

#[test]
fn check_that_range_only_contains_range() {
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
        helper.execute_expect_success(true);
    }

    let minimum = 15;
    let maximum = 25;
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    println!("vector: {:?}", vector);
    println!("Output: {:?}", output);
    let mut last = i32::MIN;
    for i in output.clone().into_iter() {
        assert!(last < i, "range_not_sorted {:?}", output.clone());
        last = i;
        assert!(vector.contains(&i), "i not contained in the tree {}", i);
        assert!(i >= 15, "All elements should be bigger 15");
        assert!(i <= 25, "All elements should be bigger 25");
    }
}

#[test]
fn check_range_mutability() {
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
        helper.execute_expect_success(true);
    }
    let minimum = 15;
    let maximum = 25;
    helper.update_values(minimum, maximum, -1);
    helper.execute_expect_success(true);
    helper.get_range(minimum, maximum);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    for i in output.into_iter() {
        assert_eq!(i, -1);
    }
    helper.get_range(maximum, 30);
    let receipt = helper.execute_expect_success(true);
    let output: Vec<Vec<i32>> = receipt.outputs("get_range");
    let output = output[0].clone();
    print!("{:?}", output);
    for (i, value) in output.into_iter().enumerate() {
        assert_eq!(i as i32 + maximum, value);
    }
}