mod helper_avl_tree;

#[cfg(test)]
mod avltree_random{
    use super::*;
    use helper_avl_tree::*;
    use scrypto::prelude::*;
    use rand::prelude::*;

    #[test]
    fn test_random() {
        for i in 0..10 {
            println!("Test {}", i);
            let mut rng = rand::thread_rng();
            let mut vector: Vec<i32> = (0..100).collect();
            vector.shuffle(&mut rng);
            let mut to_delete = vector.clone();
            to_delete.shuffle(&mut rng);
            test_range(vector, to_delete);
        }
    }
    #[test]
    fn test_with_random_function_order() {
        for i in 0..10 {
            println!("Test {}", i);
            let mut rng = rand::thread_rng();
            let mut vector: Vec<i32> = (0..100).collect();
            vector.shuffle(&mut rng);
            let mut to_delete = vector.clone();
            to_delete.shuffle(&mut rng);
            let mut functions: Vec<Function> = Vec::new();
            functions.shuffle(&mut rng);
            // test_with_functions(vector, to_delete, functions);
        }
    }
}
