mod helper_avl_tree;

#[cfg(test)]
mod avltree_insert {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_avl_tree::*;

    #[test]
    fn test_same_side_balance_left() {
        // Tests a simple balance to the left side
        // Tree after inserting 3 before balancing
        //   1
        //    2
        //     3
        // Tree after inserting 3 after balancing
        //     2
        //   1   3
        let vector: Vec<i32> = vec![1, 2, 3];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_same_side_balance_right() {
        // Tests a simple balance to the right side
        let vector: Vec<i32> = vec![3, 2, 1];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_balance_with_child_bf_zero() {
        // Test a balance where the child has a bf of 0
        // Tree after inserting 6 before balancing
        //     2|2
        //  1|0    4|1
        //        3|0  5|1
        //               6|0
        // Tree after inserting 6 after balancing
        //     4|0
        //   2|0  5|1
        //  1 3     6
        let vector: Vec<i32> = vec![2, 1, 4, 3, 5, 6];
        let to_delete = vector.clone();
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_right() {
        // Tests where the child has a different direction than the imbalance direction of the root.
        // This is the 3. case in the balance function.
        // Tree after inserting 2 before balancing
        // 1
        //    3
        //  2
        // Tree after inserting 2 after balancing
        //   2
        // 1   3
        let vector: Vec<i32> = vec![1, 3, 2];
        let to_delete = vec![3, 2, 1];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_left() {
        // Same thing other side
        // Tree after inserting 2 before balancing
        //   3
        // 1
        //  2
        // Tree after inserting 2 after balancing
        //   2
        // 1   3
        let vector: Vec<i32> = vec![3, 1, 2];
        let to_delete = vec![1, 2, 3];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_left_nr_bf_1() {
        // Test 3. case with bf of 1 for the new root in this case node 4.
        // Tree after inserting 5 before balancing
        //            2|2
        //     1            6|-1
        //             4|1       7
        //                5
        // Tree after inserting 5 after first rotation
        //            2|2
        //     1            4|-2
        //                      6|0
        //                     5    7
        // Tree after insert 5 and second rotation
        //            4|0
        //     2|0          6|-1
        //  1            5|0     7

        let vector: Vec<i32> = vec![2, 1, 6, 4, 7, 5];
        let to_delete = vec![7, 2, 1, 6, 4];
        test_range(vector, to_delete);
    }

    #[test]
    fn test_different_side_balance_left_nr_bf_minus_1() {
        // Test 3. case with bf of -1 for the new root in this case node 4.
        // Tree after inserting 3 before balancing
        //            2|2
        //     1            6|-1
        //             4|1       7
        //            3
        // Tree after inserting 3 after first rotation
        //            2|2
        //     1            4|1
        //                 3     6|0
        //                          7
        // Tree after insert 3 and second rotation
        //            4|0
        //     2|0          6|1
        //  1      3|0         7
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
