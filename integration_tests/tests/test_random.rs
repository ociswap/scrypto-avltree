mod helper_avl_tree;

#[cfg(test)]
mod avltree_random {
    use super::*;
    use helper_avl_tree::*;
    use log4rs::append::file::FileAppender;
    use log4rs::config::{Appender, Config, Root};
    use rand::prelude::*;
    use scrypto::prelude::*;
    // use test_log::test;

    #[ignore]
    #[test]
    fn test_insert_a_million_elements() {
        let mut vector: Vec<i32> = (0..1_000_000).collect();
        let mut rng = rand::thread_rng();
        vector.shuffle(&mut rng);
        let mut to_delete = vector.clone();
        to_delete.shuffle(&mut rng);
        write_costs_csv_test_range(vector, to_delete);
    }
    fn init_logger() {
        let logfile = FileAppender::builder()
            .encoder(Box::new(log4rs::encode::pattern::PatternEncoder::new(
                "",
            )))
            .build("log/error_case.log")
            .unwrap();
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(
                Root::builder()
                    .appender("logfile")
                    .build(log::LevelFilter::Info),
            );
        log4rs::init_config(config.unwrap()).unwrap();
    }
    #[test]
    fn test_with_random_function_order() {
        init_logger();
        let tree_size = 100;
        let iterations = 100;
        let mut rng = rand::thread_rng();
        for i in 0..iterations {
            println!("Test {}", i);
            let mut vector: Vec<i32> = (0..tree_size).collect();
            vector.shuffle(&mut rng);
            let mut to_delete = vector.clone();
            to_delete.shuffle(&mut rng);
            let mut functions: Vec<Function> = Vec::new();
            for i in 0..tree_size {
                functions.push(Function::Insert(i));
                functions.push(Function::Delete(i));
            }
            functions.shuffle(&mut rng);
            let result = test_with_functions(&vector, &functions);
            if result.is_err() {
                log::error!("Error: {:?}", result.err());
                log::error!("");
                log::error!("==================================================");
                log::error!("");
                log::error!("Error above here is additional information");
                log::error!("current_iteration: {}", i);
                log::error!("initial elements into tree used: {:?}", vector);
                log::error!("functions used: {:?}", functions);
                log::error!("elements to delete: {:?}", to_delete);
                panic!();
            }
        }
    }
}
