mod helper_avl_tree_tick;

#[cfg(test)]
mod avltree_stress_test {
    use super::*;
    use helper_avl_tree_tick::*;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use rand::prelude::*;
    use scrypto::prelude::*;
    // use test_log::test;

    #[test]
    fn test_insert_a_million_elements() {
        let mut vector: Vec<i32> = (0..1_000_000).collect();
        let mut rng = rand::thread_rng();
        vector.shuffle(&mut rng);
        write_costs_csv_test_range(vector);
        
    }
}
