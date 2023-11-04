mod helper_avl_tree_small;

#[cfg(test)]
mod avltree_insert {
    use scrypto::prelude::*;
    use scrypto_testenv::TestHelperExecution;
    use super::*;
    use helper_avl_tree_small::*;

    fn get_left_right_child(value:u16) -> (u16, u16) {
        let half = value * (3/4) ;
        let double = value / (4);
        (half, double)
    }
    fn insert_in_tree_rec(value: u16, helper: &mut TestHelper, inserts: &mut u16, depth: u16){
        if depth >20{
            return;
        }
        let (left, right) = get_left_right_child(value);
        *inserts = *inserts +1;
        helper.insert(left, ());
        *inserts = *inserts + 1;
        helper.insert(right, ());
        println!("depth: {}", inserts);
        helper.execute_expect_success(false);
        insert_in_tree_rec(left, helper, inserts, depth+1);
        insert_in_tree_rec(right, helper, inserts, depth+1);
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
        let mut inserts = 0;
        let start_value = u16::MAX/2-1;
        let mut queue:Vec<u16> = vec![start_value];
        let mut helper = TestHelper::new();
        helper.instantiate_default(false);

        helper.insert(start_value,());
        for i in 1..20 {
            let mut next_queue: Vec<u16> = vec![];
            for value in queue.iter(){
                let (left, right) = get_left_right_child(*value);
                helper.insert(left, ());
                helper.insert(right, ());
                helper.execute_expect_success(true);
                inserts = inserts + 2;
                next_queue.push(left);
                next_queue.push(right);
                println!("inserts: {}, depth {}", inserts, i);
            }
            println!("inserts: {}, depth {}", inserts, i);
            queue = next_queue;
        }
    }
}
