use scrypto::prelude::*;
use std::ops::Bound::{Excluded, Included};

use crate::avl_tree::AvlTree;
use crate::avl_tree_health::{check_health, print_tree_nice};

#[blueprint]
mod avl_container_decimal {

    struct AVLContainerDecimal {
        avl_tree: AvlTree<Decimal, Decimal>,
    }

    impl AVLContainerDecimal {
        pub fn instantiate() -> Global<AVLContainerDecimal> {
            let avl_tree = AvlTree::new();
            let component = Self { avl_tree }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(&mut self, key: Decimal, value: Decimal) {
            self.avl_tree.insert(key, value);
        }
        pub fn check_health(&mut self) {
            check_health(&mut self.avl_tree);
        }
        pub fn print(&mut self) {
            print_tree_nice(&mut self.avl_tree, Decimal::from(-1));
        }

        pub fn get_range_back_both_included(&mut self, key1: Decimal, key2: Decimal) -> Vec<Decimal> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Included(key2))) {
                result.push(node.clone());
            }
            result
        }
        pub fn get_range_back_both_excluded(&mut self, key1: Decimal, key2: Decimal) -> Vec<Decimal> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Excluded(key1), Excluded(key2))) {
                result.push(node.clone());
            }
            result
        }
        pub fn get_range_back(&mut self, key1: Decimal, key2: Decimal) -> Vec<Decimal> {
            let mut result = Vec::new();
            for node in self.avl_tree.range_back((Included(key1), Excluded(key2))) {
                result.push(node.clone());
            }
            result
        }
        pub fn get_range(&mut self, key1: Decimal, key2: Decimal) -> Vec<Decimal> {
            let mut result = Vec::new();
            // Standard range is Included(start) and Excluded(end)
            for node in self.avl_tree.range(key1..key2) {
                result.push(node.clone());
            }
            result
        }
        pub fn update_values(&mut self, start_key: Decimal, end_key: Decimal, new_value: Decimal) {
            self.avl_tree.range_mut(start_key..end_key).for_each(|node| {
                *node = new_value.clone();
            });
        }
        pub fn update_values_back(&mut self, start_key: Decimal, end_key: Decimal, new_value: Decimal) {
            self.avl_tree.range_back_mut(start_key..end_key).for_each(|node| {
                *node = new_value.clone();
            });
        }
        pub fn update_value(&mut self, key: Decimal, new_value: Decimal) {
            let mut test = self.avl_tree.get_mut(&key).unwrap();
            *test = new_value;
        }
        pub fn get(&mut self, key: Decimal) -> Option<Decimal> {
            self.avl_tree.get(&key).map(|x| x.clone())
        }
        pub fn delete(&mut self, key: Decimal) -> Option<Decimal>{
            self.avl_tree.delete(key)
        }
    }
}
