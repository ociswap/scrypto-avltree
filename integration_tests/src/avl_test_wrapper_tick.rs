use scrypto::prelude::*;
use std::ops::Bound::{Excluded, Included};

use crate::tick::Tick;
use scrypto_avltree::avl_tree::AvlTree;
use scrypto_avltree::avl_tree::IterMutControl;
use scrypto_avltree::avl_tree_health::{check_health, print_tree_nice};
use std::ops::RangeBounds;
fn key_value(tuple: (i32, Tick, Option<i32>)) -> (i32, Tick) {
    (tuple.0.clone(), tuple.1.clone())
}

#[blueprint]
mod avl_test_wrapper_tick {

    struct AvlTestWrapperTick {
        avl_tree: AvlTree<i32, Tick>,
    }

    impl AvlTestWrapperTick {
        pub fn instantiate() -> Global<AvlTestWrapperTick> {
            let avl_tree = AvlTree::new();
            let component = (Self { avl_tree })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
            component
        }

        pub fn insert(
            &mut self,
            key: i32,
            value: (
                PreciseDecimal,
                PreciseDecimal,
                PreciseDecimal,
                PreciseDecimal,
                PreciseDecimal,
            ),
        ) -> Option<Tick> {
            let value = Tick {
                delta_liquidity: value.0,
                total_liquidity: value.1,
                price_sqrt: value.2,
                x_fee_outside: value.3,
                y_fee_outside: value.4,
            };
            self.avl_tree.insert(key, value)
        }
        pub fn noop(&self) {
            // Do nothing
        }

        pub fn check_health(&mut self) {
            check_health(&mut self.avl_tree);
        }

        pub fn print(&mut self) {
            print_tree_nice(&mut self.avl_tree, -1);
        }
        fn range_back_with_range_bounds<R: RangeBounds<i32>>(
            &mut self,
            range: R,
        ) -> Vec<(i32, Tick)> {
            self.avl_tree.range_back(range).map(key_value).collect()
        }
        fn range_with_range_bounds<R: RangeBounds<i32>>(&mut self, range: R) -> Vec<(i32, Tick)> {
            self.avl_tree.range(range).map(key_value).collect()
        }

        pub fn get_range_back_both_included(&mut self, key1: i32, key2: i32) -> Vec<(i32, Tick)> {
            self.range_back_with_range_bounds((Included(key1), Included(key2)))
        }

        pub fn get_range_back_both_excluded(&mut self, key1: i32, key2: i32) -> Vec<(i32, Tick)> {
            self.range_back_with_range_bounds((Excluded(key1), Excluded(key2)))
        }

        pub fn get_range_back(&mut self, key1: i32, key2: i32) -> Vec<(i32, Tick)> {
            self.range_back_with_range_bounds((Included(key1), Excluded(key2)))
        }

        pub fn get_range(&mut self, key1: i32, key2: i32) -> Vec<(i32, Tick)> {
            self.range_with_range_bounds(key1..key2)
        }

        pub fn get_range_both_included(&mut self, key1: i32, key2: i32) -> Vec<(i32, Tick)> {
            self.range_with_range_bounds((Included(key1), Included(key2)))
        }
        fn range_mut_with_range_bounds<R: RangeBounds<i32>>(
            &mut self,
            range: R,
        ) -> Vec<(i32, Tick, Option<i32>)> {
            let mut result = Vec::new();
            self.avl_tree.range_mut(range).for_each(|(k, v, n)| {
                result.push((k.clone(), v.clone(), n.clone()));
                return IterMutControl::Continue;
            });
            result
        }
        fn range_mut_back_with_range_bounds<R: RangeBounds<i32>>(
            &mut self,
            range: R,
        ) -> Vec<(i32, Tick, Option<i32>)> {
            let mut result = Vec::new();
            self.avl_tree.range_back_mut(range).for_each(|(k, v, n)| {
                result.push((k.clone(), v.clone(), n.clone()));
                return IterMutControl::Continue;
            });
            result
        }
        pub fn get_range_mut_both_excluded(
            &mut self,
            key1: i32,
            key2: i32,
        ) -> Vec<(i32, Tick, Option<i32>)> {
            self.range_mut_with_range_bounds((Excluded(key1), Excluded(key2)))
        }
        pub fn get_range_mut_both_included(
            &mut self,
            key1: i32,
            key2: i32,
        ) -> Vec<(i32, Tick, Option<i32>)> {
            self.range_mut_with_range_bounds((Included(key1), Included(key2)))
        }

        pub fn get_range_back_mut_both_included(
            &mut self,
            key1: i32,
            key2: i32,
        ) -> Vec<(i32, Tick, Option<i32>)> {
            self.range_mut_back_with_range_bounds((Included(key1), Included(key2)))
        }
        pub fn get_range_back_mut_both_excluded(
            &mut self,
            key1: i32,
            key2: i32,
        ) -> Vec<(i32, Tick, Option<i32>)> {
            self.range_mut_back_with_range_bounds((Excluded(key1), Excluded(key2)))
        }
        pub fn get_range_both_excluded(&mut self, key1: i32, key2: i32) -> Vec<(i32, Tick)> {
            self.range_with_range_bounds((Excluded(key1), Excluded(key2)))
        }

        pub fn update_value(
            &mut self,
            key: i32,
            new_value: (
                PreciseDecimal,
                PreciseDecimal,
                PreciseDecimal,
                PreciseDecimal,
                PreciseDecimal,
            ),
        ) -> Option<Tick> {
            let mut test = self.avl_tree.get_mut(&key)?;
            let old_value = test.clone();
            *test = Tick {
                delta_liquidity: new_value.0,
                total_liquidity: new_value.1,
                price_sqrt: new_value.2,
                x_fee_outside: new_value.3,
                y_fee_outside: new_value.4,
            };
            Some(old_value)
        }

        pub fn get(&mut self, key: i32) -> Option<Tick> {
            self.avl_tree.get(&key).map(|x| x.clone())
        }

        pub fn remove(&mut self, key: i32) -> Option<Tick> {
            self.avl_tree.remove(&key)
        }
    }
}
