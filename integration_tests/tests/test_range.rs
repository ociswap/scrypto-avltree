mod helper_avl_tree;

#[cfg(test)]
mod avltree_range {
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
    fn range_out_of_bounds() {
        let mut helper = helper_with_initial_data((1..5).collect());
        helper.get_range_success(6, 10, vec![], true);
    }

    #[test]
    fn start_included_end_excluded() {
        let mut helper = helper_with_initial_data((1..5).collect());
        helper.get_range_success(1, 5, vec![(1, 1), (2, 2), (3, 3), (4, 4)], true);
    }

    #[test]
    fn test_range_is_sorted() {
        let mut helper = helper_with_initial_data(vec![
            13, 24, 43, 23, 12, 23, 13, 42, 53, 54, 21, 11, 12, 14, 16,
        ]);
        helper.get_range_success(
            i32::MIN,
            i32::MAX,
            vec![
                (11, 11),
                (12, 12),
                (13, 13),
                (14, 14),
                (16, 16),
                (21, 21),
                (23, 23),
                (24, 24),
                (42, 42),
                (43, 43),
                (53, 53),
                (54, 54),
            ],
            true,
        );
    }

    #[test]
    fn test_range_back_is_sorted() {
        let mut helper = helper_with_initial_data(vec![
            13, 24, 43, 23, 12, 23, 13, 42, 53, 54, 21, 11, 12, 14, 16,
        ]);
        helper.get_range_back_success(
            i32::MIN,
            i32::MAX,
            vec![
                (54, 54),
                (53, 53),
                (43, 43),
                (42, 42),
                (24, 24),
                (23, 23),
                (21, 21),
                (16, 16),
                (14, 14),
                (13, 13),
                (12, 12),
                (11, 11),
            ],
            true,
        );
    }

    #[test]
    fn test_range_back_only_contains_range() {
        let mut helper = helper_with_initial_data((10..30).collect());

        helper.get_range_back_success(
            15,
            25,
            vec![
                (24, 24),
                (23, 23),
                (22, 22),
                (21, 21),
                (20, 20),
                (19, 19),
                (18, 18),
                (17, 17),
                (16, 16),
                (15, 15),
            ],
            true,
        );

        let receipt = helper
            .get_range_back_both_included(15, 25)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(
            output,
            vec![vec![
                (25, 25),
                (24, 24),
                (23, 23),
                (22, 22),
                (21, 21),
                (20, 20),
                (19, 19),
                (18, 18),
                (17, 17),
                (16, 16),
                (15, 15)
            ]]
        );

        let receipt = helper
            .get_range_back_both_excluded(15, 25)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(
            output,
            vec![vec![
                (24, 24),
                (23, 23),
                (22, 22),
                (21, 21),
                (20, 20),
                (19, 19),
                (18, 18),
                (17, 17),
                (16, 16)
            ]]
        );
    }

    #[test]
    fn test_range_only_contains_range() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper.get_range(15, 25).execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range");
        assert_eq!(
            output,
            vec![vec![
                (15, 15),
                (16, 16),
                (17, 17),
                (18, 18),
                (19, 19),
                (20, 20),
                (21, 21),
                (22, 22),
                (23, 23),
                (24, 24)
            ]]
        );

        let receipt = helper
            .get_range_both_included(15, 25)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(
            output,
            vec![vec![
                (15, 15),
                (16, 16),
                (17, 17),
                (18, 18),
                (19, 19),
                (20, 20),
                (21, 21),
                (22, 22),
                (23, 23),
                (24, 24),
                (25, 25)
            ]]
        );

        let receipt = helper
            .get_range_both_excluded(15, 25)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(
            output,
            vec![vec![
                (16, 16),
                (17, 17),
                (18, 18),
                (19, 19),
                (20, 20),
                (21, 21),
                (22, 22),
                (23, 23),
                (24, 24)
            ]]
        );
    }

    #[test]
    fn test_range_only_contains_range_first_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_both_included(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(10, 10)]]);

        let receipt = helper
            .get_range_both_included(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(10, 10), (11, 11)]]);

        let receipt = helper
            .get_range_both_included(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(10, 10), (11, 11)]]);

        let receipt = helper
            .get_range_both_included(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(10, 10), (11, 11), (12, 12)]]);
    }

    #[test]
    fn test_range_only_contains_range_first_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_both_excluded(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_both_excluded(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![(10, 10)]]);

        let receipt = helper
            .get_range_both_excluded(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_both_excluded(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![(11, 11)]]);
    }

    #[test]
    fn test_range_only_contains_range_last_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_both_included(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(29, 29)]]);

        let receipt = helper
            .get_range_both_included(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(28, 28), (29, 29)]]);

        let receipt = helper
            .get_range_both_included(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(28, 28), (29, 29)]]);

        let receipt = helper
            .get_range_both_included(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_included");
        assert_eq!(output, vec![vec![(27, 27), (28, 28), (29, 29)]]);
    }
    #[test]
    fn test_range_only_contains_range_last_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_both_excluded(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_both_excluded(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_both_excluded(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![(29, 29)]]);

        let receipt = helper
            .get_range_both_excluded(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_both_excluded");
        assert_eq!(output, vec![vec![(28, 28)]]);
    }

    #[test]
    fn test_range_mut_only_contains_range_mut_first_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_mut_both_included(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(output, vec![vec![(10, 10, None)]]);

        let receipt = helper
            .get_range_mut_both_included(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(output, vec![vec![(10, 10, Some(11)), (11, 11, None)]]);

        let receipt = helper
            .get_range_mut_both_included(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(output, vec![vec![(10, 10, Some(11)), (11, 11, None)]]);

        let receipt = helper
            .get_range_mut_both_included(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(
            output,
            vec![vec![(10, 10, Some(11)), (11, 11, Some(12)), (12, 12, None)]]
        );
    }

    #[test]
    fn test_range_mut_only_contains_range_mut_first_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_mut_both_excluded(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_mut_both_excluded(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![(10, 10, None)]]);

        let receipt = helper
            .get_range_mut_both_excluded(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_mut_both_excluded(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![(11, 11, None)]]);
    }

    #[test]
    fn test_range_mut_only_contains_range_mut_last_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_mut_both_included(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(output, vec![vec![(29, 29, None)]]);

        let receipt = helper
            .get_range_mut_both_included(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(output, vec![vec![(28, 28, Some(29)), (29, 29, None)]]);

        let receipt = helper
            .get_range_mut_both_included(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(output, vec![vec![(28, 28, Some(29)), (29, 29, None)]]);

        let receipt = helper
            .get_range_mut_both_included(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_included");
        assert_eq!(
            output,
            vec![vec![(27, 27, Some(28)), (28, 28, Some(29)), (29, 29, None)]]
        );
    }

    #[test]
    fn test_range_mut_only_contains_range_last_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_mut_both_excluded(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_mut_both_excluded(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_mut_both_excluded(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![(29, 29, None)]]);

        let receipt = helper
            .get_range_mut_both_excluded(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_mut_both_excluded");
        assert_eq!(output, vec![vec![(28, 28, None)]]);
    }

    // TODO add same tests for range_back

    #[test]
    fn test_range_back_only_contains_range_first_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_both_included(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(10, 10)]]);

        let receipt = helper
            .get_range_back_both_included(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(11, 11), (10, 10)]]);

        let receipt = helper
            .get_range_back_both_included(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(11, 11), (10, 10)]]);

        let receipt = helper
            .get_range_back_both_included(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(12, 12), (11, 11), (10, 10)]]);
    }

    #[test]
    fn test_range_back_only_contains_range_first_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_both_excluded(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_both_excluded(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![(10, 10)]]);

        let receipt = helper
            .get_range_back_both_excluded(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_both_excluded(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![(11, 11)]]);
    }

    #[test]
    fn test_range_back_only_contains_range_last_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_both_included(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(29, 29)]]);

        let receipt = helper
            .get_range_back_both_included(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(29, 29), (28, 28)]]);

        let receipt = helper
            .get_range_back_both_included(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(29, 29), (28, 28)]]);

        let receipt = helper
            .get_range_back_both_included(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_included");
        assert_eq!(output, vec![vec![(29, 29), (28, 28), (27, 27)]]);
    }

    #[test]
    fn test_range_back_only_contains_range_last_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_both_excluded(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_both_excluded(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_both_excluded(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![(29, 29)]]);

        let receipt = helper
            .get_range_back_both_excluded(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32)>> = receipt.outputs("get_range_back_both_excluded");
        assert_eq!(output, vec![vec![(28, 28)]]);
    }
    #[test]
    fn test_range_back_mut_only_contains_range_first_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_mut_both_included(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(output, vec![vec![(10, 10, None)]]);

        let receipt = helper
            .get_range_back_mut_both_included(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(output, vec![vec![(11, 11, Some(10)), (10, 10, None)]]);

        let receipt = helper
            .get_range_back_mut_both_included(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(output, vec![vec![(11, 11, Some(10)), (10, 10, None)]]);

        let receipt = helper
            .get_range_back_mut_both_included(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(
            output,
            vec![vec![(12, 12, Some(11)), (11, 11, Some(10)), (10, 10, None)]]
        );
    }

    #[test]
    fn test_range_back_mut_only_contains_range_first_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_mut_both_excluded(9, 10)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_mut_both_excluded(9, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![(10, 10, None)]]);

        let receipt = helper
            .get_range_back_mut_both_excluded(10, 11)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_mut_both_excluded(10, 12)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![(11, 11, None)]]);
    }

    #[test]
    fn test_range_back_mut_only_contains_range_last_included() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_mut_both_included(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(output, vec![vec![(29, 29, None)]]);

        let receipt = helper
            .get_range_back_mut_both_included(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(output, vec![vec![(29, 29, Some(28)), (28, 28, None)]]);

        let receipt = helper
            .get_range_back_mut_both_included(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(output, vec![vec![(29, 29, Some(28)), (28, 28, None)]]);

        let receipt = helper
            .get_range_back_mut_both_included(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_included");
        assert_eq!(
            output,
            vec![vec![(29, 29, Some(28)), (28, 28, Some(27)), (27, 27, None)]]
        );
    }

    #[test]
    fn test_range_back_mut_only_contains_range_last_excluded() {
        let mut helper = helper_with_initial_data((10..30).collect());

        let receipt = helper
            .get_range_back_mut_both_excluded(29, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_mut_both_excluded(28, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![]]);

        let receipt = helper
            .get_range_back_mut_both_excluded(28, 30)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![(29, 29, None)]]);

        let receipt = helper
            .get_range_back_mut_both_excluded(27, 29)
            .execute_expect_success(true);
        let output: Vec<Vec<(i32, i32, Option<i32>)>> =
            receipt.outputs("get_range_back_mut_both_excluded");
        assert_eq!(output, vec![vec![(28, 28, None)]]);
    }

    #[test]
    fn test_range_after_mutating() {
        let mut helper = helper_with_initial_data((10..30).collect());
        helper
            .update_values(15, 25, -1)
            .execute_expect_success(true);
        helper.get_range_success(
            15,
            25,
            vec![
                (15, -1),
                (16, -1),
                (17, -1),
                (18, -1),
                (19, -1),
                (20, -1),
                (21, -1),
                (22, -1),
                (23, -1),
                (24, -1),
            ],
            true,
        );
        helper.get_range_success(
            25,
            30,
            vec![(25, 25), (26, 26), (27, 27), (28, 28), (29, 29)],
            true,
        );
    }

    #[test]
    fn test_range_after_mutating_with_max_iters() {
        let mut helper = helper_with_initial_data((10..30).collect());
        helper
            .update_values_max_iters(15, 25, 5, -1)
            .execute_expect_success(true);
        helper.get_range_success(
            15,
            25,
            vec![
                (15, -1),
                (16, -1),
                (17, -1),
                (18, -1),
                (19, -1),
                (20, 20),
                (21, 21),
                (22, 22),
                (23, 23),
                (24, 24),
            ],
            true,
        );
        helper.get_range_success(
            25,
            30,
            vec![(25, 25), (26, 26), (27, 27), (28, 28), (29, 29)],
            true,
        );
    }
}
