use scrypto::prelude::*;
use scrypto_avltree::avl_tree::AvlTree;
use scrypto_avltree::avl_tree_health::{check_health, print_tree_nice};

#[blueprint]
mod hello_tree {
    use std::ops::Bound::{Excluded, Included};

    struct HelloTree {
        tree: AvlTree<Decimal, String>,
    }

    impl HelloTree {
        pub fn instantiate() -> Global<HelloTree> {
            (Self {
                tree: AvlTree::new(),
            })
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn fancy_operations(&mut self) {
            /* Calculate some shenanigan add stuff and remove stuff from tree */
            self.tree.insert(dec!(1), "Hello".to_string());
            self.tree.insert(dec!(2), "World".to_string());
            // check_health(&mut self.tree);
            self.tree.insert(dec!(3), "!".to_string());
            self.tree.insert(dec!(3.5), "How".to_string());
            self.tree.insert(dec!(4), "are".to_string());
            self.tree.insert(dec!(5), "you".to_string());
            self.tree.insert(dec!(1000), "doing".to_string());
            self.tree.remove(&dec!(3.5));
            self.tree.remove(&dec!(4));
            // Override value 1
            self.tree.insert(dec!(1), "New Hello".to_string());
            let range = self.tree.range(dec!(1)..dec!(5));
            let special_range = self.tree.range((Excluded(dec!(1)), Included(dec!(5))));
            for (key, value, next_key) in range {
                // print " New Hello World ! you", since items are sorted.
                // "are" and "you" are deleted, and "doing" is not in range
                info!("{} ", value);
            }
            self.tree.range_mut(dec!(1)..dec!(5)).for_each(
                |(key, value, next_key): (&Decimal, &mut String, Option<Decimal>)| {
                    info!("{} ", value);
                    scrypto_avltree::IterMutControl::Continue
                },
            );
            self.tree
                .range(dec!(1)..dec!(5))
                .map(|(key, _v, next_key)| next_key.map(|next_key| key + next_key))
                .for_each(|(new_value)| {
                    info!("{} ", new_value);
                });
            /* more fancy operations */
        }
    }
}
