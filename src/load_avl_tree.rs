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
        pub fn check_health(&mut self) {
            check_health(&mut self.avl_tree);
        }
        pub fn print(&mut self) {
            print_tree_nice(&mut self.avl_tree);
        }

        pub fn get_range_back_both_included(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Included(key2))) {
                result.push(node.value.clone());
            }
            result
        }
        pub fn get_range_back_both_excluded(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Excluded(key1), Excluded(key2))) {
                result.push(node.value.clone());
            }
            result
        }
        pub fn get_range_back(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Excluded(key2))) {
                result.push(node.value.clone());
            }
            result
        }
        pub fn get_range(&mut self, key1: i32, key2: i32) -> Vec<i32> {
            let mut result = Vec::new();
            // Standard range is Included(start) and Excluded(end)
            for node in self.avl_tree.range(key1 .. key2) {
                result.push(node.value.clone());
            }
            result
        }
        pub fn update_values(&mut self, start_key: i32, end_key: i32, new_value: i32) {
        debug!("{}",end_key);
            self.avl_tree.range_mut(start_key .. end_key).for_each(|node| {
                debug!("{}", node.value);
                node.value = new_value;
            });
        }
        pub fn delete(&mut self, key: i32) {
            self.avl_tree.delete(key);
        }
    }
}
