use scrypto::prelude::*;
use std::ops::Bound::{ Excluded, Included };

use crate::avl_tree::AvlTree;
use crate::avl_tree::IterMutControl;

#[blueprint]
mod avl_test_wrapper_u16 {
    struct AvlTestWrapperU16 {
        avl_tree: AvlTree<u16, ()>,
    }

    impl AvlTestWrapperU16 {
        pub fn instantiate() -> Global<AvlTestWrapperU16> {
            let avl_tree = AvlTree::new();
            let component = (Self { avl_tree })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(&mut self, key: u16, value: ()) {
            self.avl_tree.insert(key, value);
        }

        // pub fn check_health(&mut self) {
        //     check_health(&mut self.avl_tree);
        // }

        // pub fn print(&mut self) {
        //     print_tree_nice(&mut self.avl_tree, u16::from(-1));
        // }
        //
        // pub fn get_range_back_both_included(
        //     &mut self,
        //     key1: u16,
        //     key2: u16
        // ) -> Vec<((), ())> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.range_back((Included(key1), Included(key2))) {
        //         result.push(node.clone());
        //     }
        //     result
        // }
        //
        // pub fn get_range_back_both_excluded(
        //     &mut self,
        //     key1: u16,
        //     key2: u16
        // ) -> Vec<((), ())> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.range_back((Excluded(key1), Excluded(key2))) {
        //         result.push(node.clone());
        //     }
        //     result
        // }
        //
        // pub fn get_range_back(&mut self, key1: u16, key2: u16) -> Vec<((), ())> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.range_back((Included(key1), Excluded(key2))) {
        //         result.push(node.clone());
        //     }
        //     result
        // }
        //
        // pub fn get_range(&mut self, key1: u16, key2: u16) -> Vec<((), ())> {
        //     let mut result = Vec::new();
        //     // Standard range is Included(start) and Excluded(end)
        //     for node in self.avl_tree.range(key1..key2) {
        //         result.push(node.clone());
        //     }
        //     result
        // }
        //
        // pub fn get_range_both_included(
        //     &mut self,
        //     key1: u16,
        //     key2: u16
        // ) -> Vec<((), ())> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.range((Included(key1), Included(key2))) {
        //         result.push(node.clone());
        //     }
        //     result
        // }
        //
        // pub fn get_range_both_excluded(
        //     &mut self,
        //     key1: u16,
        //     key2: u16
        // ) -> Vec<((), ())> {
        //     let mut result = Vec::new();
        //     for node in self.avl_tree.range((Excluded(key1), Excluded(key2))) {
        //         result.push(node.clone());
        //     }
        //     result
        // }

        pub fn update_values(&mut self, start_key: u16, end_key: u16, new_value: ()) {
            self.avl_tree.range_mut(start_key..end_key).for_each(|_, value| {
                *value = new_value.clone();
                return IterMutControl::Continue;
            });
        }

        pub fn update_values_back(
            &mut self,
            start_key: u16,
            end_key: u16,
            new_value: ()
        ) {
            self.avl_tree.range_back_mut(start_key..end_key).for_each(|_, value| {
                *value = new_value.clone();
                return IterMutControl::Continue;
            });
        }

        pub fn update_value(&mut self, key: u16, new_value: ()) {
            let mut test = self.avl_tree.get_mut(&key).unwrap();
            *test = new_value;
        }

        pub fn get(&mut self, key: u16) -> Option<()> {
            self.avl_tree.get(&key).map(|x| x.clone())
        }

        pub fn delete(&mut self, key: u16) -> Option<()> {
            self.avl_tree.delete(&key)
        }
    }
}
