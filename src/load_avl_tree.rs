use scrypto::prelude::*;
use std::ops::Bound::{Excluded, Included};

use crate::avl_tree::AvlTree;
use crate::avl_tree_health::{check_health, print_tree_nice};

#[blueprint]
mod avl_container {
    use std::ops::Bound::{Excluded, Included};

    struct AVLContainer {
        avl_tree: AvlTree<i32, i32>,
    }

    impl AVLContainer {
        pub fn instantiate() -> Global<AVLContainer> {
            let avl_tree = AvlTree::new();
            let component = Self { avl_tree }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(&mut self, key: i32, value: i32) {
            self.avl_tree.insert(key, value);
        }
        pub fn check_health(&self) {
            check_health(&self.avl_tree);
        }
        pub fn print(&self) {
            print_tree_nice(&self.avl_tree);
        }
        pub fn get_value(&mut self, key1: i32) -> i32 {
            self.avl_tree.get_mut(&key1).unwrap().value = 3;
            self.avl_tree.get_mut(&key1).unwrap().value
        }

        pub fn get_range_back_both_included(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Included(key2))) {
                result.push(node.value);
            }
            result
        }
        pub fn get_range_back_both_excluded(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Excluded(key1), Excluded(key2))) {
                result.push(node.value);
            }
            result
        }
        pub fn get_range_back(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Excluded(key2))) {
                result.push(node.value);
            }
            result
        }
        pub fn get_range(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            // Standard range is Included(start) and Excluded(end)
            for node in self.avl_tree.range(key1 .. key2) {
                result.push(node.value);
            }
            result
        }
        pub fn delete(&mut self, key: i32) {
            self.avl_tree.delete(key);
        }
    }
}
