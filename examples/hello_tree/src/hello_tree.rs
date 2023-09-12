use scrypto::prelude::*;
use avl_tree::AvlTree;

#[blueprint]
mod hello_tree {
    use std::ops::Bound;
    use std::ops::Bound::{Excluded, Included};

    struct HelloTree {
        tree: AvlTree<Decimal, String>,
    }

    impl HelloTree {
        pub fn instantiate(
        ) -> (Global<HelloSwap>, Decimal) {
            let component = Self {
                tree: AvlTree::new(),
            }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();

            component
        }

        pub fn fancy_operations(&mut self)  {
            /* Calculate some shenanigan add stuff and remove stuff from tree */
            self.tree.insert(Decimal::from(1), "Hello".to_string());
            self.tree.insert(Decimal::from(2), "World".to_string());
            self.tree.insert(Decimal::from(3), "!".to_string());
            self.tree.insert(Decimal::from(3.5), "How".to_string());
            self.tree.insert(Decimal::from(4), "are".to_string());
            self.tree.insert(Decimal::from(5), "you".to_string());
            self.tree.insert(Decimal::from(1000), "doing".to_string());
            self.tree.delete(Decimal::from(3.5));
            self.tree.delete(Decimal::from(4));
            // Override value 1
            self.tree.insert(Decimal::from(1), "New Hello".to_string());
            let range = self.tree.range(Decimal::from(1).. Decimal::from(5));
            let special_range = self.tree.range((Excluded(Decimal::from(1)),Included(Decimal::from(5))));
            assert_eq!(range.len(), 4);
            for node in range {
                // print " New Hello World ! you", since items are sorted.
                // "are" and "you" are deleted, and "doing" is not in range
                println!("{} ", node.value);
            }
            /* more fancy operations */
        }
    }
}