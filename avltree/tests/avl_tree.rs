mod test_utils;
use crate::test_utils::test_range;
// Debug tests:
#[test]
fn test_increasing() {
    let vector: Vec<i32> = (0..20).collect();
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}
#[test]
fn deletion_with_2_parents_above_but_only_one_balance() {
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut to_delete = vec![0, 1, 2, 3];
    test_range(vector, to_delete);
}

#[test]
fn deletion_with_replace_direct_below() {
    let vector: Vec<i32> = vec![15, 14, 17, 16];
    let mut to_delete = vec![17];
    test_range(vector, to_delete);
}

#[test]
fn delete_and_balance_at_root() {
    let vector: Vec<i32> = vec![7, 5, 15, 3, 6, 11, 17, 4, 16, 18, 20, 14];
    let mut to_delete = vec![3];
    test_range(vector, to_delete);
}

#[test]
fn test_decrease() {
    let vector: Vec<i32> = (0..20).rev().collect();
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}


#[test]
fn replace_has_children() {
    let vector = vec![48, 27, 81, 13, 40, 72, 35];
    let mut to_delete = vec![48];
    test_range(vector, to_delete);
}

#[test]
fn replace_has_children_other_direction() {
    let vector = vec![48, 27, 81, 93, 71, 40, 73];
    let mut to_delete = vec![48];
    test_range(vector, to_delete);
}

#[test]
fn replace_has_to_change_balance() {
    let vector = vec![74, 11, 48, 27, 90, 35, 82, 48, 10, 12, 59, 72, 46, 40, 13, 81, 93, 21, 30, 37, 23, 42, 29, 22, 98, 31, 31, 14, 73];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}

#[test]
fn test_more_than_two_balances_in_delete() {
    let vector = vec![25, 20, 30, 10, 23, 26, 33, 31];
    let mut to_delete = vec![25];
    test_range(vector, to_delete);
}

#[test]
fn delete_is_bf_0_but_not_shorten() {
    let vector = vec![44, 39, 49, 36, 42, 46, 51, 34, 40, 43, 47, 52, 41];
    let mut to_delete = vec![36];
    test_range(vector, to_delete);
}


#[test]
fn replace_with_left_child_different_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 72, 75, 73];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

#[test]
fn replace_with_left_child_same_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 73, 75, 71];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

#[test]
fn replace_with_right_child_same_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 73, 75, 76];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

#[test]
fn replace_with_right_childdifferent_direction() {
    // was wrong wired in the leftover child of right child in this case the 76 had the wrong parent
    let vector = vec![74, 73, 76, 75];
    let mut to_delete = vec![74];
    test_range(vector, to_delete);
}

// #[test]
fn test_random() {
    let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6, 59, 72, 46, 46, 8, 81, 93, 64, 98, 11, 92, 10, 26, 34, 20, 13, 0, 42, 70, 87, 94, 2, 60, 14, 39, 18, 77, 41, 56, 15, 75, 79, 57, 33, 32, 21, 83, 100, 31, 9, 66, 88, 63, 30, 19, 37, 17, 28, 51, 67, 53, 4, 24, 44, 95, 38, 52, 71, 29, 36, 89, 3, 73, 84, 80, 43, 55, 91, 50, 76, 22, 49, 86, 47, 23, 7, 58, 54, 16, 25, 12, 68, 61, 96, 97, 65, 78, 45, 85, 69, 62];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}

#[test]
fn remove_last_inserted() {
    let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6];
    let mut to_delete = vec![6];
    test_range(vector, to_delete);
}

#[test]
fn replace_2_layers_above() {
    // rewiring of node in the middle could go wrong because it has to be in memory.
    let vector: Vec<i32> = vec![18, 15, 21, 12, 16, 20];
    let mut to_delete = vec![18];
    test_range(vector, to_delete);
}

#[test]
fn replace_node_has_not_bf_0_after_rewire() {
    let vector: Vec<i32> = vec![18, 15, 21, 12, 16, 20];
    let mut to_delete = vec![16];
    test_range(vector, to_delete);
}

#[test]
fn test_balancing_of_subtree_with_different_directions() {
    // Test balancing of subtree with different direction.
    let vector = vec![10, 25, 11, 13, 15, 12, 18, 20, 21, 22];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}
