mod helper_avl_tree;

#[cfg(test)]
mod avltree_random {
    use super::*;
    use helper_avl_tree::*;
    use rand::prelude::*;
    use scrypto::prelude::*;

    #[test]
    fn test_random() {
        let iterations = 10;
        let vector_size = 100;
        for i in 0..iterations {
            println!("Test {}", i);
            let mut rng = rand::thread_rng();
            let mut vector: Vec<i32> = (0..vector_size).collect();
            vector.shuffle(&mut rng);
            let mut to_delete = vector.clone();
            to_delete.shuffle(&mut rng);
            test_range(vector, to_delete);
        }
    }
    #[test]
    fn test_with_random_function_order() {
        let iterations = 10;
        let vector_size = 100;
        for i in 0..iterations {
            println!("Test {}", i);
            let mut rng = rand::thread_rng();
            let mut vector: Vec<i32> = (0..vector_size).collect();
            vector.shuffle(&mut rng);
            let mut functions: Vec<Function> = (0..vector_size)
                .map(|i| vec![Function::Insert(i), Function::Delete(i)])
                .flatten()
                .collect();
            functions.shuffle(&mut rng);
            test_with_functions(vector, functions);
        }
    }
}
