mod helper_avl_tree;

#[cfg(test)]
mod avltree_delete {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_avl_tree::*;

    #[test]
    fn replace_still_in_range() {
        let mut vec = vec![26, 18, 34, 14, 20, 30, 38, 12, 16, 22, 28, 32, 36, 40];
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        for i in vec.clone() {
            helper.insert(i, i);
            helper.check_health();
            helper.execute_expect_success(true);
        }
        helper.delete(26);
        helper.check_health();
        helper.execute_expect_success(true);

        vec.remove(0);
        vec.sort();
        helper.get_range_success(i32::MIN, i32::MAX, to_key_values(&vec), true);
    }

    #[test]
    fn test_delete_root() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        helper.insert(1, 1);
        helper.check_health();
        helper.execute_expect_success(true);
        helper.delete(1);
        helper.check_health();
        helper.execute_expect_success(true);
        helper.get_range_success(i32::MIN, i32::MAX, vec![], true);
    }

    #[test]
    fn test_get_before_and_after_delete() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        helper.insert(1, 400);
        helper.check_health();
        helper.get(1);
        let recipt = helper.execute_expect_success(true);
        let output: Vec<Option<i32>> = recipt.outputs("get");
        let output = output[0].clone();
        assert_eq!(output, Some(400), "Something is still present in the tree");
        helper.delete(1);
        helper.check_health();
        helper.get(1);
        let recipt = helper.execute_expect_success(true);
        let delete_output: Vec<Option<i32>> = recipt.outputs("delete");
        let delete_output = delete_output[0].clone();
        let get_output: Vec<Option<i32>> = recipt.outputs("get");
        let get_output = get_output[0].clone();
        assert_eq!(delete_output, Some(400), "One was deleted from tree and returned");
        assert_eq!(get_output, None, "One was deleted from tree");
        helper.delete(1);
        helper.check_health();
        let recipt = helper.execute_expect_success(true);
        let delete_output: Vec<Option<i32>> = recipt.outputs("delete");
        let delete_output = delete_output[0].clone();
        assert_eq!(
            delete_output,
            None,
            "Delete did not return None after deleting non existent element"
        );
    }

    #[test]
    fn three_insert_one_delete_3_insert() {
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        let mut insert = vec![];
        let mut delete = vec![];
        let mut should_be_in_tree = HashMap::new();
        for i in 0..4 {
            for mut j in 0..3 {
                j = 3 - j;
                let key = i * 3 + j;
                let value = key;
                print!("insert: --------------------- {}, {}, {} ", value, i, j);
                helper.insert(key, value);
                should_be_in_tree.insert(key, value);
                insert.push(value);
                helper.check_health();
                helper.execute_expect_success(true);
            }
            let key = i * 2 + 1;
            helper.delete(key);
            should_be_in_tree.remove(&key);
            delete.push(key);
            helper.check_health();
            helper.execute_expect_success(true);
        }
        let mut should_be_in_tree: Vec<(i32, i32)> = should_be_in_tree.into_iter().collect();
        should_be_in_tree.sort();
        println!("insert: {:?}", insert);
        println!("delete: {:?}", delete);
        helper.get_range_success(i32::MIN, i32::MAX, should_be_in_tree, true);
    }

    #[test]
    fn shorten_was_calculated_wrong_because_balance_factor_of_delete_was_wrong() {
        let vector: Vec<i32> = vec![5, 3, 7, 1, 4, 8, 2];
        let to_delete = vec![5];
        test_range(vector, to_delete);
    }

    #[test]
    fn delete_root_and_check_if_replace_parent_is_given_correct() {
        // If 2 has the wrong bf afterwards the parent was given incorrect
        let vec = vec![6, 2, 7, 3];
        let to_delete = vec![6];
        test_range(vec, to_delete);
    }

    #[test]
    fn replace_jumps_over_his_parent_with_rebalance_other_direction() {
        let vec = vec![16, 12, 18, 11, 13, 17, 10];
        let to_delete = vec![12];
        test_range(vec, to_delete);
    }

    #[test]
    fn replace_jumps_over_his_parent_with_rebalance() {
        let vec = vec![16, 12, 18, 11, 13, 17, 10];
        let to_delete = vec![12];
        test_range(vec, to_delete);
    }

    #[test]
    fn replace_jumps_overhis_parent() {
        let vec = vec![6, 2, 7, 1, 3];
        let to_delete = vec![6];
        test_range(vec, to_delete);
    }

    #[test]
    fn delet_non_existent_and_dont_panic() {
        let vec = vec![6, 2];
        let to_delete = vec![8];
        test_range(vec, to_delete);
    }

    #[test]
    fn deletion_with_2_parents_above_but_only_one_balance() {
        let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let to_delete = vec![0, 1, 2, 3];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_more_than_two_balances_in_delete() {
        let vector = vec![25, 20, 30, 10, 23, 26, 33, 31];
        let to_delete = vec![25];
        test_range(vector, to_delete);
    }

    #[test]
    fn delete_is_bf_0_but_not_shorten() {
        let vector = vec![44, 39, 49, 36, 42, 46, 51, 34, 40, 43, 47, 52, 41];
        let to_delete = vec![36];
        test_range(vector, to_delete);
    }

    #[test]
    fn deletion_with_replace_direct_below() {
        let vector: Vec<i32> = vec![15, 14, 17, 16];
        let to_delete = vec![17];
        test_range(vector, to_delete);
    }

    #[test]
    fn delete_and_balance_at_root() {
        let vector: Vec<i32> = vec![7, 5, 15, 3, 6, 11, 17, 4, 16, 18, 20, 14];
        let to_delete = vec![3];
        test_range(vector, to_delete);
    }
}
