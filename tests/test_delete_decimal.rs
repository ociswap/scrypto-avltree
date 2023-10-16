mod helper_avl_tree_decimal;

#[cfg(test)]
mod avltree_delete_decimal {
    use scrypto::prelude::*;
    use super::*;
    use helper_avl_tree_decimal::*;

    #[test]
    fn test_delete_increasing_numbers() {
        let vector: Vec<Decimal> = (0..20).map(|x| Decimal::from(x)).collect();
        let mut to_delete = vec![];
        for i in 0..vector.len() {
            to_delete.push(vector[i]);
        }
        test_range(vector, to_delete);
    }

    #[test]
    fn test_delete_decreasing_numbers() {
        let vector: Vec<Decimal> = (0..20)
            .rev()
            .map(|x| Decimal::from(x))
            .collect();
        let mut to_delete = vec![];
        for i in 0..vector.len() {
            to_delete.push(vector[i]);
        }
        test_range(vector, to_delete);
    }
}
