use scrypto::prelude::*;

use crate::avl_tree::AvlTree;

#[blueprint]
mod avl_container {
    use scrypto::debug;

    struct AVLContainer {
        avl_tree: AvlTree<i32>,
    }

    impl AVLContainer {
        pub fn instantiate() -> Global<AVLContainer> {
            let avl_tree = AvlTree::new();
            // let dummy_component = Self { avl_tree }.instantiate();
            // let dummy_component_address = dummy_component.globalize();
            // dummy_component_address
            let component = Self {
                avl_tree
            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        // pub fn insert(&mut self, key: i32, value: i32) {
        //     self.avl_tree.insert(key, value);
        // }
        // pub fn check_health(&self)  {
        //     self.avl_tree.check_health()
        // }
        // pub fn print(&self)  {
        //     self.avl_tree.print_tree_nice()
        // }
        // pub fn get_range(&mut self, key1: i32, key2: i32) -> Vec<i32> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.get_range(key1, key2){
        //         result.push(node.value);
        //     }
        //     result
        // }
        // pub fn get_range_mut(&mut self, key1: i32, key2: i32) -> Vec<i32> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.get_range_mut(key1, key2){
        //         result.push(node.value);
        //     }
        //     result
        // }
        // pub fn delete(&mut self, key: i32) {
        //     self.avl_tree.delete(key);
        // }
    }
}