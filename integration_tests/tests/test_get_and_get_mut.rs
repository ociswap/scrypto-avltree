mod helper_avl_tree;

#[cfg(test)]
mod avltree_get_and_get_mut {
    use super::*;
    use helper_avl_tree::*;
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;

    fn helper_with_initial_data(vector: Vec<i32>) -> TestHelper {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        for i in vector.iter() {
            helper.insert(*i, *i);
            helper.check_health();
            helper.execute_expect_success(false);
        }
        helper
    }

    #[test]
    fn test_get_mut() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper.update_value(9, 1232132).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("update_value");
        assert_eq!(output, vec![None]);
        helper.insert(1, 10002132).execute_expect_success(true);
        let receipt = helper.update_value(1, -1).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("update_value");
        assert_eq!(output, vec![Some(10002132)]);
        helper.insert(-2132123, 2132).execute_expect_success(true);
        let receipt = helper.update_value(1, -2).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("update_value");
        assert_eq!(output, vec![Some(-1)]);
        let receipt = helper.update_value(9, 1232132).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("update_value");
        assert_eq!(output, vec![None]);
        helper.insert(9, -3).execute_expect_success(true);
        let receipt = helper.update_value(9, 1232132).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("update_value");
        assert_eq!(output, vec![Some(-3)]);
    }

    #[test]
    fn test_get() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper.get(9).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("get");
        assert_eq!(output, vec![None]);
        helper.insert(1, 10002132);
        let receipt = helper.get(1).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("get");
        assert_eq!(output, vec![Some(10002132)]);
        helper.insert(-2132123, 2132);
        let receipt = helper.get(1).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("get");
        assert_eq!(output, vec![Some(10002132)]);
        let receipt = helper.get(2132123).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("get");
        assert_eq!(output, vec![None]);
        let receipt = helper.get(-2132124).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("get");
        assert_eq!(output, vec![None]);
        let receipt = helper.get(-2132123).execute_expect_success(true);
        let output: Vec<Option<i32>> = receipt.outputs("get");
        assert_eq!(output, vec![Some(2132)]);
    }
}
