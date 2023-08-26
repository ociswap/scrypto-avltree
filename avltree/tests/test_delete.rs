use pretty_assertions::assert_eq;
use scrypto::prelude::*;
mod helper_avl_tree;
use helper_avl_tree::*;

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
