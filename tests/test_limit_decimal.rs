mod helper_avl_tree_decimal;

#[cfg(test)]
mod avltree_insert {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_avl_tree_decimal::*;

    fn get_left_right_child(value:Decimal) -> (Decimal, Decimal) {
        let half = value * Decimal::from(3/4) ;
        let double = value / Decimal::from(4);
        (half, double)
    }
    fn insert_in_tree_rec(value: Decimal, helper: &mut TestHelper, inserts: &mut Decimal, depth: u32){
        if depth >20{
            return;
        }
        let (left, right) = get_left_right_child(value);
        *inserts = *inserts + Decimal::ONE;
        helper.insert(left, inserts.clone());
        *inserts = *inserts + Decimal::ONE;
        helper.insert(right, inserts.clone());
        println!("depth: {}", inserts);
        helper.execute_expect_success(false);
        insert_in_tree_rec(left, helper, inserts, depth+1);
        insert_in_tree_rec(right, helper, inserts, depth+1);
    }

    // #[test]
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
        let start_value: Decimal= Decimal::MAX/2-Decimal::ONE;
        let start_value: Decimal = start_value.checked_floor().unwrap();
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        helper.insert(start_value,start_value);
        let mut inserts = Decimal::ZERO;
        insert_in_tree_rec(start_value, &mut helper, &mut inserts, 0);
        panic!()
    }

    #[test]
    fn test_same_side_balance_left_queue() {
        // Tests a simple balance to the left side
        // Tree after inserting 3 before balancing
        //   1
        //    2
        //     3
        // Tree after inserting 3 after balancing
        //     2
        //   1   3
        let mut inserts = Decimal::ZERO;
        let start_value: Decimal= Decimal::MAX/2-Decimal::ONE;
        let mut queue:Vec<Decimal> = vec![start_value];
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        for i in (1..20) {
            let mut next_queue: Vec<Decimal> = vec![];
            for value in queue.iter(){
                let (left, right) = get_left_right_child(*value);
                helper.insert(left, inserts.clone());
                helper.insert(right, inserts.clone());
                helper.execute_expect_success(true);
                inserts = inserts + Decimal::from(2);
                next_queue.push(left);
                next_queue.push(right);
                println!("inserts: {}, depth {}", inserts, i);
            }
            println!("inserts: {}, depth {}", inserts, i);
            queue = next_queue;
        }
        let start_value: Decimal = start_value.checked_floor().unwrap();
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);
        helper.insert(start_value,start_value);
        insert_in_tree_rec(start_value, &mut helper, &mut inserts, 0);
        panic!()
    }
}
