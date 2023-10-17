use scrypto::prelude::*;
use std::ops::Bound::{ Excluded, Included };

use crate::avl_tree::AvlTree;
use crate::avl_tree::IterMutControl;
use crate::avl_tree_health::{ check_health, print_tree_nice };

#[blueprint]
mod avl_test_wrapper {
    struct AvlTestWrapper {
        avl_tree: AvlTree<i32, i32>,
    }

    impl AvlTestWrapper {
        pub fn instantiate() -> Global<AvlTestWrapper> {
            let avl_tree = AvlTree::new();
            let component = (Self { avl_tree })
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
            print_tree_nice(&mut self.avl_tree, -1);
        }

        pub fn get_range_back_both_included(&mut self, key1: i32, key2: i32) -> Vec<(i32, i32)> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Included(key2))) {
                result.push(node.clone());
            }
            result
        }

        pub fn get_range_back_both_excluded(&mut self, key1: i32, key2: i32) -> Vec<(i32, i32)> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Excluded(key1), Excluded(key2))) {
                result.push(node.clone());
            }
            result
        }

        pub fn get_range_back(&mut self, key1: i32, key2: i32) -> Vec<(i32, i32)> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Excluded(key2))) {
                result.push(node.clone());
            }
            result
        }

        pub fn get_range(&mut self, key1: i32, key2: i32) -> Vec<(i32, i32)> {
            let mut result = Vec::new();
            // Standard range is Included(start) and Excluded(end)
            for node in self.avl_tree.range(key1..key2) {
                result.push(node.clone());
            }
            result
        }

        pub fn get_range_both_included(&mut self, key1: i32, key2: i32) -> Vec<(i32, i32)> {
            let mut result = Vec::new();
            for node in self.avl_tree.range((Included(key1), Included(key2))) {
                result.push(node.clone());
            }
            result
        }

        pub fn get_range_both_excluded(&mut self, key1: i32, key2: i32) -> Vec<(i32, i32)> {
            let mut result = Vec::new();
            for node in self.avl_tree.range((Excluded(key1), Excluded(key2))) {
                result.push(node.clone());
            }
            result
        }

        pub fn update_values(&mut self, start_key: i32, end_key: i32, new_value: i32) {
            self.avl_tree.range_mut(start_key..end_key).for_each(|_, value| {
                *value = new_value;
                return IterMutControl::Continue;
            });
        }

        pub fn update_values_back(&mut self, start_key: i32, end_key: i32, new_value: i32) {
            self.avl_tree.range_back_mut(start_key..end_key).for_each(|_, value| {
                *value = new_value;
                return IterMutControl::Continue;
            });
        }
        pub fn update_values_max_iters(&mut self, start_key: i32, end_key: i32, max_iters:i32, new_value: i32) {
            let mut count = 0;
            self.avl_tree.range_mut(start_key..end_key).for_each(|_, value| {
                *value = new_value;
                return if count < max_iters {
                    count += 1;
                    IterMutControl::Continue
                } else {
                    IterMutControl::Break
                }
            });
        }

        pub fn update_value(&mut self, key: i32, new_value: i32) {
            let mut test = self.avl_tree.get_mut(&key).unwrap();
            *test = new_value;
        }

        pub fn get(&mut self, key: i32) -> Option<i32> {
            self.avl_tree.get(&key).map(|x| *x)
        }

        pub fn delete(&mut self, key: i32) -> Option<i32> {
            self.avl_tree.delete(&key)
        }
    }
}
