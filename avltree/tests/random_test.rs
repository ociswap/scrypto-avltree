mod test_utils;
use crate::test_utils::test_range;

// #[test]
fn test_random2() {
    // let vector = vec![74, 5, 48, 27, 90, 35, 82, 99, 1, 6, 59, 72, 46, 46, 8, 81, 93, 64, 98, 11, 92, 10, 26, 34, 20, 13, 0, 42, 70, 87, 94, 2, 60, 14, 39, 18, 77, 41, 56, 15, 75, 79, 57, 33, 32, 21, 83, 100, 31, 9, 66, 88, 63, 30, 19, 37, 17, 28, 51, 67, 53, 4, 24, 44, 95, 38, 52, 71, 29, 36, 89, 3, 73, 84, 80, 43, 55, 91, 50, 76, 22, 49, 86, 47, 23, 7, 58, 54, 16, 25, 12, 68, 61, 96, 97, 65, 78, 45, 85, 69, 62];
    let vector = vec![14, 53, 63, 96, 66, 74, 12, 48, 87, 60, 59, 67, 58, 75, 76, 23, 38, 16, 79, 32, 27, 37, 88, 78, 50, 26, 45, 93, 22, 35, 36, 98, 46, 18, 57, 81, 82, 30, 73, 61, 34, 83, 77, 84, 25, 69, 29, 89, 95, 19, 13, 33, 97, 55, 49, 51, 20, 21, 42, 54, 94, 90, 62, 43, 15, 40, 71, 86, 92, 99, 64, 39, 28, 70, 72, 24, 65, 44, 47, 56, 91, 11, 52, 68, 17, 41, 31, 80, 85, 10];
    let mut to_delete = vec![];
    for i in 0..vector.len() / 2 {
        to_delete.push(vector[i]);
    }
    test_range(vector, to_delete);
}

// #[test]
fn test_random3() {
    let vector = vec![14, 60, 59, 95, 20, 42, 86, 39, 57, 98, 74, 34, 68, 29, 91, 92, 36, 56, 66, 50, 62, 58, 11, 37, 15, 52, 38, 17, 12, 79, 89, 53, 16, 65, 25, 64, 30, 97, 23, 24, 87, 10, 94, 44, 67, 76, 47, 61, 75, 81, 70, 26, 71, 72, 54, 35, 41, 27, 51, 84, 73, 55, 63, 13, 93, 31, 82, 69, 96, 80, 45, 49, 18, 32, 33, 28, 40, 99, 22, 43, 21, 77, 83, 90, 88, 19, 48, 78, 46, 85];
    let to_delete = vector.clone();
    test_range(vector, to_delete);
}