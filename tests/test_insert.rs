mod helper_avl_tree;

#[cfg(test)]
mod avltree_insert {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_avl_tree::*;

    #[test]
    fn test_same_side_balance_left() {
        let vector: Vec<i32> = vec![1, 2, 3];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_same_side_balance_right() {
        let vector: Vec<i32> = vec![3, 2, 1];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_balance_wtih_child_zero_bf_right() {
        let vector: Vec<i32> = vec![2, 1, 3];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_right() {
        let vector: Vec<i32> = vec![1, 3, 2];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_left() {
        let vector: Vec<i32> = vec![3, 1, 2];
        let to_delete = vec![1, 2, 3];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_left_nr_bf_1() {
        let vector: Vec<i32> = vec![2, 1, 6, 4, 7, 5];
        let to_delete = vec![7, 2, 1, 6, 4];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_left_nr_bf_minus_1() {
        let vector: Vec<i32> = vec![2, 1, 6, 4, 7, 3];
        let to_delete = vec![7, 3, 2, 1, 6, 4];
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
            helper.execute_expect_success(true);
        }
        for i in vector.iter() {
            helper.insert(*i, 0);
            helper.check_health();
            helper.execute_expect_success(true);
        }

        let output_expected = vector
            .iter()
            .zip(vec![0; 20].iter())
            .map(|(a, b)| (*a, *b))
            .collect();
        helper.get_range_success(i32::MIN, i32::MAX, output_expected, true);
    }

    #[test]
    fn remove_last_inserted() {
        let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6];
        let to_delete = vec![6];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_2_layers_above() {
        // rewiring of node in the middle could go wrong because it has to be in memory.
        let vector: Vec<i32> = vec![18, 15, 21, 12, 16, 20];
        let to_delete = vec![18];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_node_has_not_bf_0_after_rewire() {
        let vector: Vec<i32> = vec![18, 15, 21, 12, 16, 20];
        let to_delete = vec![16];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_with_left_child_different_direction() {
        // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
        let vector = vec![74, 72, 75, 73];
        let to_delete = vec![74];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_with_left_child_same_direction() {
        // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
        let vector = vec![74, 73, 75, 71];
        let to_delete = vec![74];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_with_right_child_same_direction() {
        // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
        let vector = vec![74, 73, 75, 76];
        let to_delete = vec![74];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_with_right_childdifferent_direction() {
        // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
        let vector = vec![74, 73, 76, 75];
        let to_delete = vec![74];
        test_range(vector, to_delete);
    }

    #[test]
    fn replace_has_children() {
        let vector = vec![48, 27, 81, 13, 40, 72, 35];
        let to_delete = vec![48];
        test_range(vector, to_delete);
    }
}
