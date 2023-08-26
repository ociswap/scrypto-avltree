use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::VecDeque;

use scrypto::prelude::*;

#[derive(ScryptoSbor, Clone)]
pub struct Node<T: ScryptoSbor> {
    key: i32,
    pub value: T,
    left: Option<i32>,
    right: Option<i32>,
    parent: Option<i32>,
    next: Option<i32>,
    prev: Option<i32>,
    balance_factor: i32,
}

impl<T: ScryptoSbor> Node<T> {
    fn set_child(&mut self, direction: Direction, child: Option<i32>) {
        match direction {
            Direction::Left => self.left = child,
            Direction::Right => self.right = child,
        }
    }
    fn replace_child(&mut self, old_child: i32, new_child: Option<i32>) {
        if self.left == Some(old_child) {
            self.left = new_child;
        } else if self.right == Some(old_child) {
            self.right = new_child;
        } else {
            panic!("Tried to over ride {} but was not a child of {}", old_child, self.key);
        }
    }
    fn get_child(&self, direction: Direction) -> Option<i32> {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
    fn has_child(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }
    fn get_imbalance_direction(&self) -> Option<Direction> {
        Direction::from_balance_factor(self.balance_factor)
    }
    fn set_prev_next(&mut self, direction: Direction, node: Option<i32>) {
        match direction {
            Direction::Left => self.prev = node,
            Direction::Right => self.next = node,
        }
    }
    fn get_prev_next(&self, direction: Direction) -> Option<i32> {
        match direction {
            Direction::Left => self.prev,
            Direction::Right => self.next,
        }
    }
    fn remove_child(&mut self, key: i32) {
        if self.left == Some(key) {
            self.left = None;
        } else if self.right == Some(key) {
            self.right = None;
        } else {
            panic!("Tried to remove child which did not exist!")
        }
    }
    fn direction_from_parent(&self) -> Option<Direction> {
        let parent = self.parent?;
        let direction = Direction::from_ordering(parent.cmp(&self.key)).expect("Nodes should be unequal");
        Some(direction)
    }
    fn direction_from_other(&self, other: i32) -> Option<Direction> {
        Some(Direction::from_ordering(other.cmp(&self.key)).expect("Nodes should be unequal"))
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}


impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
    fn is_over(&self, value: i32, other: i32) -> bool {
        match self {
            Self::Left => value < other,
            Self::Right => value > other,
        }
    }

    fn direction_factor(&self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }
    fn from_balance_factor(balance_factor: i32) -> Option<Self> {
        match balance_factor.signum() {
            -1 => Some(Self::Left),
            1 => Some(Self::Right),
            _ => None
        }
    }
    fn from_ordering(ordering: Ordering) -> Option<Self> {
        match ordering {
            Less => Some(Self::Left),
            Greater => Some(Self::Right),
            Equal => None
        }
    }
    fn get_next<T: ScryptoSbor>(&self, node: &KeyValueEntryRef<Node<T>>) -> Option<i32> {
        match self {
            Self::Left => node.prev,
            Self::Right => node.next,
        }
    }
    fn get_next_mut<T: ScryptoSbor>(&self, node: &KeyValueEntryRefMut<Node<T>>) -> Option<i32> {
        match self {
            Self::Left => node.prev,
            Self::Right => node.next,
        }
    }
}


pub struct NodeIterator<'a, T: ScryptoSbor> {
    current: Option<i32>,
    direction: Direction,
    end: Option<i32>,
    store: &'a KeyValueStore<i32, Node<T>>,
}


impl<'a, T: ScryptoSbor> Iterator for NodeIterator<'a, T> {
    type Item = KeyValueEntryRef<'a, Node<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.store.get(&self.current?).expect("Node not found");
        if self.direction.is_over(node.key, self.end.expect("End is not set")) {
            self.current = None;
            self.end = None;
            return None;
        }
        self.current = self.direction.get_next(&node);
        Some(node)
    }
}

// pub struct NodeIteratorMut<'a, T: ScryptoSbor> {
//     current: Option<i32>,
//     direction: Direction,
//     end: Option<i32>,
//     store: &'a mut KeyValueStore<i32, Node<T>>,
// }
//
// impl<'a, T: ScryptoSbor> Iterator for NodeIteratorMut<'a, T> {
//     type Item = KeyValueEntryRefMut<'a, Node<T>>;
//     fn next(self: &'_ mut NodeIteratorMut<'a, T>) -> Option<Self::Item> {
//
//         let k = 1;
//         let node: KeyValueEntryRefMut<Node<T>> = self.store.get_mut(&k).expect("Node not found");
//         if self.direction.is_over(node.key, self.end.expect("End is not set")) {
//             self.current = None;
//             self.end = None;
//             return None;
//         }
//         self.current = self.direction.get_next_mut(&node);
//         Some(node)
//     }
// }

#[derive(ScryptoSbor)]
pub struct AvlTree<T: ScryptoSbor> {
    root: Option<i32>,
    store: KeyValueStore<i32, Node<T>>,
}

impl<T: ScryptoSbor> AvlTree<T> where T: Clone {
    pub fn new() -> Self {
        AvlTree {
            root: None,
            store: KeyValueStore::new(),
        }
    }

    fn get_node(&self, key: Option<i32>) -> KeyValueEntryRef<Node<T>> {
        let data_ref = self.store.get(&key.expect("Call on empty tree"));
        match data_ref {
            None => panic!("Call on empty tree {}", key.unwrap()),
            Some(data) => data
        }
    }

    fn get_mut_node(&mut self, key: i32) -> KeyValueEntryRefMut<Node<T>> {
        let data_ref = self.store.get_mut(&key);
        match data_ref {
            None => panic!("Call on empty tree {}", key),
            Some(data) => data
        }
    }

    // pub fn get_range(&self, start_key: i32, end_key: i32) -> NodeIterator<T> {
    //     assert!(start_key <= end_key, "Start key should be smaller than end key");
    //     let mut start = None;
    //     if start.is_none() {
    //         let start_key = self.find_first_right_node(start_key);
    //         start = start_key.map(|k| self.store.get(&k).expect("Node of subtree should exist."));
    //     }
    //     NodeIterator {
    //         current: start.map(|n| n.key),
    //         direction: Direction::Right,
    //         end: Some(end_key),
    //         store: &self.store,
    //     }
    // }
    //
    // pub fn get_range_mut(&mut self, start_key: i32, end_key: i32) -> NodeIteratorMut<T> {
    //     let mut start = None;
    //     if start.is_none() {
    //         let start_key = self.find_first_right_node(start_key);
    //         start = start_key.map(|k| self.store.get_mut(&k).expect("Node of subtree should exist."));
    //     }
    //     NodeIteratorMut {
    //         current: start.map(|n| n.key),
    //         direction: Direction::Right,
    //         end: Some(end_key),
    //         store: &mut self.store,
    //     }
    // }
    //
    // fn find_first_right_node(&self, lower_bound: i32) -> Option<i32> {
    //     let mut current = self.root;
    //     let mut result = None;
    //     while current.is_some() {
    //         let node = self.store.get(&current.unwrap()).expect("Node of subtree should exist.");
    //         if node.key < lower_bound {
    //             current = node.right;
    //         } else {
    //             result = current;
    //             current = node.left;
    //         }
    //     }
    //     result
    // }

    // pub fn depth(&self) -> usize {
    //     self.depth_of_node(self.root)
    // }
    // fn depth_of_node(&self, key: Option<i32>) -> usize {
    //     if key.is_none() {
    //         return 0;
    //     }
    //     let node = self.get_node(key);
    //     let left_depth = self.depth_of_node(node.left);
    //     let right_depth = self.depth_of_node(node.right);
    //     1 + left_depth.max(right_depth)
    // }

//     // Insert functions
//
//     pub fn insert(&mut self, key: i32, value: T) {
//         let (mut parents, mut child) = self.insert_node_in_empty_spot(key, value);
//         let mut deepen = true;
//         let mut node_tuple = parents.pop();
//         let parent_indices = parents.into_iter().map(|(parent, dir)| (parent.key, dir.clone())).collect::<Vec<(i32, Direction)>>();
//         parents = parent_indices.into_iter().map(|(key, dir)| (self.get_mut_node(key), dir)).collect::<Vec<(KeyValueEntryRefMut<Node<T>>, Direction)>>();
//         while let Some((mut node, direction)) = node_tuple {
//             let (mut parent, parent_direction) = parents.pop().unzip();
//             if true {
//                 deepen = node.balance_factor == 0;
//                 node.balance_factor += direction.direction_factor();
//             }
//             if node.balance_factor.abs() == 2 {
//                 self.balance(&mut node, child, parent.as_mut(), direction);
//             }
//             child = node;
//             node_tuple = parent.zip(parent_direction);
//         }
//     }
//
//     fn insert_node_in_empty_spot(&mut self, key: i32, value: T) -> (Vec<(KeyValueEntryRefMut<Node<T>>, Direction)>, KeyValueEntryRefMut<Node<T>>) {
//         let mut parents = match self.calculate_insert_parent_list(key) {
//             Ok(parents) => parents,
//             Err(mut same_node) => {
//                 same_node.value = value;
//                 return (vec![], same_node);
//             }
//         };
//         let last_tuple = parents.pop();
//
//         match last_tuple {
//             Some((mut last, dir)) => {
//                 self.insert_node_and_adjust_pointers(&mut last, key, value, dir, &mut parents);
//                 parents.push((last, dir));
//                 (parents, self.get_mut_node(key))
//             }
//             None => {
//                 // Tree is empty
//                 self.add_node(None, key, value, None, None);
//                 self.root = Some(key);
//                 (vec![], self.get_mut_node(key))
//             }
//         }
//     }
//
//     fn calculate_insert_parent_list(&mut self, key: i32) -> Result<Vec<(KeyValueEntryRefMut<Node<T>>, Direction)>, KeyValueEntryRefMut<Node<T>>> {
//         let mut parents: Vec<(KeyValueEntryRefMut<Node<T>>, Direction)> = vec![];
//         // For root and inserted node, it does not matter what direction we choose, we don't use it
//         // parents.push((self.get_mut_node(self.root.unwrap()), Direction::from_ordering(key.cmp(&self.root.unwrap()))));
//         let mut node = self.root;
//         while let Some(current_node) = node {
//             // get last element of parents
//             let mut current_root = self.get_mut_node(current_node);
//             let down_direction = Direction::from_ordering(key.cmp(&current_root.key));
//             if let Some(down_direction) = down_direction {
//                 let _node = self.get_mut_node(current_node);
//                 // reset parents list if balance_factor is not 0
//                 if _node.balance_factor != 0 {
//                     while parents.len() > 2 {
//                         parents.remove(0);
//                     }
//                 }
//                 parents.push((_node, down_direction.clone()));
//
//                 node = current_root.get_child(down_direction);
//             } else {
//                 // Key already exists do not balance parents just override value
//                 return Err(current_root);
//             }
//         };
//         Ok(parents)
//     }

    // fn add_node(&mut self, parent: Option<i32>, key: i32, value: T, prev: Option<i32>, next: Option<i32>) {
    //     self.store.insert(key, Node {
    //         key,
    //         value,
    //         left: None,
    //         right: None,
    //         next,
    //         prev,
    //         parent,
    //         balance_factor: 0,
    //     });
    // }
    //
    // fn insert_node_and_adjust_pointers(&mut self, mut last: &mut KeyValueEntryRefMut<Node<T>>, key: i32, value: T, dir: Direction, parents: &mut Vec<(KeyValueEntryRefMut<Node<T>>, Direction)>) {
    //     let parent_key = last.key;
    //     let other_neighbour: Option<i32>;
    //     // one neighbour is always the parent and the other is the next or prev of the parent, depending on the direction.
    //     other_neighbour = last.get_prev_next(dir);
    //     if let Some(neighbour_key) = other_neighbour {
    //         let mut changed = false;
    //         // Adjust other neighbour prev/next pointer, if other neighbour is in the parent list, we have to use the parent list because else the changes are overwritten afterwards from the dropping of the parent vector.
    //         for (p, _) in parents.iter_mut() {
    //             if neighbour_key == p.key {
    //                 p.set_prev_next(dir.opposite(), Some(key));
    //                 changed = true;
    //             }
    //         }
    //         if !changed {
    //             let mut node = self.get_mut_node(neighbour_key);
    //             node.set_prev_next(dir.opposite(), Some(key));
    //         }
    //     }
    //     last.set_prev_next(dir, Some(key));
    //
    //     last.set_child(dir.clone(), Some(key));
    //     let prev;
    //     let next;
    //     if let Some(neighbour) = other_neighbour {
    //         prev = Some(parent_key.min(neighbour));
    //         next = Some(parent_key.max(neighbour));
    //     } else {
    //         if dir == Direction::Left {
    //             prev = None;
    //             next = Some(parent_key);
    //         } else {
    //             prev = Some(parent_key);
    //             next = None;
    //         }
    //     }
    //     self.add_node(Some(last.key), key, value, prev, next);
    // }


    // delete functions
//     pub fn delete(&mut self, key: i32) -> Option<T> {
//         // Remove mut if store can remove nodes.
//         let mut del_node = self.store.get_mut(&key)?;
//         let (start_tuple, shortened) = self.rewire_tree_for_delete(del_node);
//
//         self.balance_tree_after_delete(start_tuple, shortened);
//         self.store.remove(&key).map(|n| n.value)
//     }
//
//     fn balance_tree_after_delete(&mut self, mut node_tuple: Option<(KeyValueEntryRefMut<Node<T>>, Direction)>, mut shortened: bool) {
//         while let Some((mut current_node, child_dir)) = node_tuple {
//             if shortened {
//                 let mut parent = current_node.parent.map(|key| self.get_mut_node(key));
//                 current_node.balance_factor += child_dir.direction_factor();
//                 // get balance direction before balancing because the parent can change afterwards.
//                 let balance_child_direction = current_node.direction_from_parent();
//                 let mut new_root_balance_factor = None;
//
//                 if current_node.balance_factor.abs() == 2 {
//                     let child = current_node.get_child(child_dir).map(|c| self.get_mut_node(c));
//                     new_root_balance_factor = Some(self.balance(&mut current_node, child.expect("there should be a child!"), parent.as_mut(), child_dir));
//                 }
//                 // continue going up if bf is 0 after removing of child -> layer was removed!
//                 shortened = new_root_balance_factor.unwrap_or(current_node.balance_factor) == 0;
//                 node_tuple = parent.zip(balance_child_direction);
//             } else {
//                 break;
//             }
//         }
//     }
//
//     fn rewire_tree_for_delete(&mut self, del_node: KeyValueEntryRefMut<Node<T>>) -> (Option<(KeyValueEntryRefMut<Node<T>>, Direction)>, bool) {
//         let mut replace_parent = None;
//         let mut direction = del_node.direction_from_parent();
//         let mut shorten = true;
//         let mut parent = del_node.parent.map(|key| self.get_mut_node(key));
//         self.rewire_next_and_previous(&del_node, parent.as_mut());
//         let mut replace_node = self.get_replace_node(&del_node);
//         // rewire parent.
//         if let Some(original_parent_node) = parent.as_mut() {
//             original_parent_node.replace_child(del_node.key, replace_node.as_ref().map(|n| n.key));
//         }
//         if let Some(mut replace) = replace_node.as_mut() {
//             // remove replace from tree.
//             let replace_parent_key = replace.parent.expect("should have parent because it is a child of current");
//             let non_empty_child = self.rewire_possible_children_in_delete(del_node, replace);
//             // rewire parent of replace.
//             if del_node.key != replace_parent_key {
//                 (replace_parent, direction) = self.delete_rewire_parent(&mut replace, replace_parent_key, non_empty_child, &del_node);
//             } else {
//                 // if parent is node to delete, we do not have to rewrite stuff because node will be lost anyway.
//                 // change balance factor of replace because will not be in the parent chain.
//                 replace.balance_factor = del_node.balance_factor + replace.direction_from_other(del_node.key).expect("Should have different keys").direction_factor();
//                 shorten = replace.balance_factor == 0;
//             }
//             self.rewire_replace_child(Direction::Left, &del_node, &mut replace_parent, replace);
//             self.rewire_replace_child(Direction::Right, &del_node, &mut replace_parent, replace);
//
//             replace.parent = del_node.parent;
//         }
//         if self.root == Some(del_node.key) {
//             self.root = replace_node.as_ref().map(|n| n.key);
//         }
//         (replace_parent.or(parent).zip(direction), shorten)
//     }
//
//     fn rewire_next_and_previous(&mut self, del_node: &KeyValueEntryRefMut<Node<T>>, mut parent: Option<&mut KeyValueEntryRefMut<Node<T>>>) {
//         // we have to use the parent if next or previous is it.
//         if parent.as_ref().map(|p| p.key) == del_node.next {
//             parent.as_mut().map(|next| next.prev = del_node.prev);
//         } else {
//             let mut next = del_node.next.map(|next_key| self.get_mut_node(next_key));
//             next.as_mut().map(|next| next.prev = del_node.prev);
//         }
//         if parent.as_ref().map(|p| p.key) == del_node.prev {
//             parent.as_mut().map(|prev| prev.next = del_node.next);
//         } else {
//             let mut prev = del_node.prev.map(|prev_key| self.get_mut_node(prev_key));
//             prev.as_mut().map(|prev| prev.next = del_node.next);
//         }
//     }
//
//     fn rewire_replace_child(&mut self, direction: Direction, del_node: &KeyValueEntryRefMut<Node<T>>, replace_parent: &mut Option<KeyValueEntryRefMut<Node<T>>>, replace: &mut KeyValueEntryRefMut<Node<T>>) {
//         let del_child_node = del_node.get_child(direction);
//         if del_child_node != Some(replace.key) {
//             replace.set_child(direction, del_child_node);
//             // careful replace parent is already in memory don't overwrite it double!
//             if del_child_node == replace.parent {
//                 replace_parent.as_mut().map(|parent| parent.parent = Some(replace.key));
//             } else {
//                 del_child_node.map(|k| self.get_mut_node(k).parent = Some(replace.key));
//             }
//         }
//     }
//
//     fn rewire_possible_children_in_delete(&mut self, del_node: KeyValueEntryRefMut<Node<T>>, replace: &KeyValueEntryRefMut<Node<T>>) -> Option<i32> {
//         let non_empty_child = replace.left.or(replace.right);
//         // rewire possible child of replace if replace and del_node are not parent and child.
//         if replace.parent != Some(del_node.key) {
//             if let Some(mut non_empty_child) = non_empty_child.map(|k| self.store.get_mut(&k).expect("Node of subtree should exist.")) {
//                 non_empty_child.parent = replace.parent;
//             }
//         }
//         non_empty_child
//     }
//
//     fn delete_rewire_parent(&mut self, replace: &mut KeyValueEntryRefMut<Node<T>>, replace_parent_key: i32, non_empty_child: Option<i32>, del_node: &KeyValueEntryRefMut<Node<T>>) -> (Option<KeyValueEntryRefMut<Node<T>>>, Option<Direction>) {
//         let mut replace_parent = self.get_mut_node(replace_parent_key);
//         replace_parent.remove_child(replace.key);
//         let mut replace_parent = Some(replace_parent);
//         let direction = replace.direction_from_parent();
//         // replace should max have one child so we have to rewire the leftover child:
//         replace_parent.as_mut().map(|n| n.set_child(direction.expect("Should have parent").opposite(), non_empty_child));
//         replace.balance_factor = del_node.balance_factor;
//         (replace_parent, direction)
//     }
//
//     fn get_replace_node(&mut self, node: &KeyValueEntryRefMut<Node<T>>) -> Option<KeyValueEntryRefMut<Node<T>>> {
//         if node.has_child() {
//             // Only needs replacement if node has a child.
//             let imbalance_direction = node.get_imbalance_direction();
//             let imbalance_next = imbalance_direction.map(|d| node.get_prev_next(d).unwrap());
//             let replace_key = imbalance_next.or_else(|| node.next.map_or(node.prev, |n| Some(n)));
//             replace_key.map(|k| self.store.get_mut(&k).expect("Node of subtree should exist."))
//         } else {
//             None
//         }
//     }
//
//     // balance functions
//     fn balance(&mut self, root: &mut KeyValueEntryRefMut<Node<T>>, mut child: KeyValueEntryRefMut<Node<T>>, parent: Option<&mut KeyValueEntryRefMut<Node<T>>>, child_direction: Direction) -> i32 {
//         // let imbalance_direction = Direction::from_balance_factor(root.balance_factor).expect("Balance factor should be -2 or 2");
//         // assert_eq!(child_direction, imbalance_direction, "Child direction {:?} should be the same as imbalance direction {:?}. Wrong child was given.", child_direction, imbalance_direction);
//         // assert!(child.balance_factor.abs() <= 1, "Subtree {:?} of {} should not have a higher balance factor than 1", child.key, root.key);
//         if child.balance_factor.signum() == child_direction.direction_factor() {
//             self.balance_with_subtree_in_same_direction(root, child, parent, child_direction)
//         } else if child.balance_factor == 0 {
//             self.balance_with_zero_bf_subtree(root, child, parent, child_direction)
//         } else {
//             self.balance_with_subtree_in_different_direction(root, child, parent, child_direction)
//         }
//     }
//
//     fn balance_with_subtree_in_same_direction(&mut self, root: &mut KeyValueEntryRefMut<Node<T>>, mut child: KeyValueEntryRefMut<Node<T>>, parent: Option<&mut KeyValueEntryRefMut<Node<T>>>, imbalance_direction: Direction) -> i32 {
//         /*
//             *  Before Balance:
//             *      R
//             *    /  \
//             *   A    C
//             *       / \
//             *      A   L
//             *         / \
//             *        A   A
//             *
//             *  After Balance
//             *     C
//             *    / \
//             *   R   L
//             *  / \ / \
//             * A  A A  A
//          */
//         child.balance_factor = 0;
//         root.balance_factor = 0;
//         self.rotate(imbalance_direction.opposite(), root, &mut child, parent);
//         child.balance_factor
//     }
//     fn balance_with_zero_bf_subtree(&mut self, root: &mut KeyValueEntryRefMut<Node<T>>, mut child: KeyValueEntryRefMut<Node<T>>, parent: Option<&mut KeyValueEntryRefMut<Node<T>>>, imbalance_direction: Direction) -> i32 {
//         /*
//                 * imbalance direction = right
//                 *  Before Balance :
//                 *   B.depth + 1 == A.depth -> C.bf = 0
//                 *
//                 *      R
//                 *    /  \
//                 *   A    C
//                 *       / \
//                 *      B   L
//                 *         / \
//                 *        A   A
//                 *
//                 *  After Balance
//                 *     C
//                 *    / \
//                 *   R   L
//                 *  / \ / \
//                 * A  B A  A
//                 * -> C.bf = -1, R.bf = -1
//                 *
//         */
//         root.balance_factor = imbalance_direction.direction_factor();
//         child.balance_factor = imbalance_direction.opposite().direction_factor();
//         self.rotate(imbalance_direction.opposite(), root, &mut child, parent);
//         child.balance_factor
//     }
//     fn balance_with_subtree_in_different_direction(&mut self, root: &mut KeyValueEntryRefMut<Node<T>>, mut child: KeyValueEntryRefMut<Node<T>>, parent: Option<&mut KeyValueEntryRefMut<Node<T>>>, imbalance_direction: Direction) -> i32 {
//         /*
//         * imbalance direction = right
//         *  Before Balance :
//         *
//         *
//         *      R
//         *    /  \
//         *   A    C
//         *       / \
//         *      NR  A
//         *     / \
//         *    B   C
//         *
//         *  After Balance
//         *     NR
//         *    / \
//         *   R   C
//         *  / \ / \
//         * A  B C  A
//         * -> Balance factor of R and C depend on old balance factor of NR
//         */
//         // This reference is not synced with the parents list. However, this child node should be further down in the tree and not in the parents list
//         let mut new_root = self.get_mut_node(child.get_child(imbalance_direction.opposite()).unwrap());
//         child.balance_factor = 0;
//         root.balance_factor = 0;
//         // If new root was balanced in the same direction, root has a child more
//         if new_root.balance_factor == imbalance_direction.direction_factor() {
//             root.balance_factor = imbalance_direction.opposite().direction_factor();
//         }
//         // If new root was balanced in the same direction, child has a child more
//         if new_root.balance_factor == imbalance_direction.opposite().direction_factor() {
//             child.balance_factor = imbalance_direction.direction_factor();
//         }
//         // reset new_root balance factor here because the old value is needed before
//         new_root.balance_factor = 0;
//         {
//             self.rotate(imbalance_direction, &mut child, &mut new_root, Some(root));
//         }{
//             self.rotate(imbalance_direction.opposite(), root, &mut new_root, parent);
//         }
//         new_root.balance_factor
//     }
//
//
//     fn rotate(&mut self, rotate_direction: Direction, root: &mut KeyValueEntryRefMut<Node<T>>, child: &mut KeyValueEntryRefMut<Node<T>>, parent: Option<&mut KeyValueEntryRefMut<Node<T>>>) {
//         /*
//             *  Rotate left:
//             *      R
//             *    /  \
//             *   _    C
//             *      / \
//             *     RL  _
//             *
//             * to:
//             *     C
//             *    / \
//             *   R   _
//             *  / \
//             * _   RL
//
//             or
//             *  Rotate right:
//             *      R
//             *    /  \
//             *   l    _
//             *  / \
//             * _   lr
//             to:
//             *     L
//             *    / \
//             *   _   R
//             *      / \
//             *     LR  _
//          */
//         let parent_key = parent.map(|p| self.rotate_rewire_parent(p, root, child));
//         if parent_key.is_none() {
//             self.root = Some(child.key);
//         }
//         child.parent = parent_key;
//
//         // Rewire root and child
//         let old_root_child = child.get_child(rotate_direction);
//         root.set_child(rotate_direction.opposite(), old_root_child);
//
//         if let Some(old_root_child_key) = old_root_child {
//             let mut old_root_child_node = self.store.get_mut(&old_root_child_key).expect("No child");
//             old_root_child_node.parent = Some(root.key);
//         }
//
//         child.set_child(rotate_direction, Some(root.key));
//         root.parent = Some(child.key);
//     }
//
//     fn rotate_rewire_parent(&self, parent: &mut KeyValueEntryRefMut<Node<T>>, root: &mut KeyValueEntryRefMut<Node<T>>, child: &mut KeyValueEntryRefMut<Node<T>>) -> i32 {
//         if parent.left == Some(root.key) {
//             parent.left = Some(child.key);
//         } else if parent.right == Some(root.key) {
//             parent.right = Some(child.key);
//         } else {
//             panic!("Parent {} is not parent of node {} tree is wired wrong", parent.key, root.key);
//         }
//         parent.key
//     }
//
//
//     // Debugging functions
//     pub fn check_health(&self) {
//         self.check_health_rec(self.root, true);
//     }
//     fn check_health_rec(&self, key: Option<i32>, panic: bool) -> (i32, Option<i32>) {
//         if key.is_none() {
//             return (0, None);
//         }
//         let node = self.store.get(&key.unwrap()).expect("Node of subtree should exist.");
//         let left = node.left;
//         let right = node.right;
//         let (height_left, parent_left) = self.check_health_rec(left, panic);
//         let (height_right, parent_right) = self.check_health_rec(right, panic);
//         assert_eq!(parent_left, node.left.map(|_| node.key), "Parent of left child of node {} is not correct.", node.key);
//         assert_eq!(parent_right, node.right.map(|_| node.key), "Parent of right child of node {} is not correct.", node.key);
//         let balance_factor = height_right - height_left;
//         if balance_factor != node.balance_factor {
//             if panic {
//                 panic!("Balance factor of node {} is not correct. Should be {} but is {}", node.key, balance_factor, node.balance_factor);
//             } else {
//                 debug!("Balance factor of node {} is not correct. Should be {} but is {}", node.key, balance_factor, node.balance_factor);
//             }
//         }
//         if balance_factor.abs() > 1 {
//             if panic {
//                 panic!("Balance factor is too high for node {}.", node.key);
//             } else {
//                 debug!("Balance factor is too high for node {}.", node.key);
//             }
//         }
//         (height_left.max(height_right) + 1, node.parent)
//     }
//
//     pub fn print_tree_nice(&self) {
//         // Works best if keys are between 10 and 99 because of formatting.
//         let mut levels: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
//         let mut queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
//         if self.root.is_none() {
//             debug!("Empty tree");
//             return;
//         }
//         queue.push_back((self.root.unwrap(), 0, 0)); // root is at depth 0, position 0.
//
//
//         while let Some((node_key, depth, pos)) = queue.pop_front() {
//             let node = self.get_node(Some(node_key));
//
//             if !levels.contains_key(&depth) {
//                 levels.insert(depth, HashMap::new());
//             }
//
//             levels.get_mut(&depth).unwrap().insert(pos, node_key);
//             // debug!("Node {} at depth {} and position {}", node_key, depth, pos); use this, when there is a loop in the tree -> infinite depth
//
//             if let Some(left) = node.left {
//                 queue.push_back((left, depth + 1, pos * 2));
//             }
//             if let Some(right) = node.right {
//                 queue.push_back((right, depth + 1, pos * 2 + 1));
//             }
//         }
//
//         let max_depth = levels.keys().max().unwrap().clone();
//         let mut spacing = " ".to_string();
//         let mut half_spacing = "".to_string();
//         // Now we print the tree.
//         let mut layers_string = Vec::new();
//         for depth in 0..max_depth + 1 {
//             let depth = max_depth - depth;
//             let level = levels.get(&depth).unwrap();
//
//             let mut node_keys: Vec<String> = Vec::new();
//             let mut balance_factors: Vec<String> = Vec::new();
//             let mut parents: Vec<String> = Vec::new();
//             let mut nexts: Vec<String> = Vec::new();
//             let mut prevs: Vec<String> = Vec::new();
//
//             for pos in 0..=2.pow(depth as u32) as i32 - 1 {
//                 if let Some(&node_key) = level.get(&pos) {
//                     let node = self.get_node(Some(node_key));
//                     node_keys.push(format!("{}", node.key.to_string()));
//                     let balance_factor = match node.balance_factor {
//                         2 => "+2",
//                         1 => "+1",
//                         0 => "+0",
//                         -1 => "-1",
//                         -2 => "-2",
//                         _ => "??"
//                     };
//                     balance_factors.push(format!("{}", balance_factor));
//                     parents.push(format!("{}", node.parent.unwrap_or(-1).to_string()));
//                     nexts.push(format!("{}", node.next.unwrap_or(-1).to_string()));
//                     prevs.push(format!("{}", node.prev.unwrap_or(-1).to_string()));
//                 } else {
//                     node_keys.push("--".to_string());
//                     parents.push("--".to_string());
//                     balance_factors.push("--".to_string());
//                     nexts.push("--".to_string());
//                     prevs.push("--".to_string());
//                 }
//             }
//             let spacing_front = match depth {
//                 _ if depth == max_depth => "".to_string(),
//                 _ => half_spacing.clone()
//             };
//
//             layers_string.push(spacing_front.clone() + nexts.join(spacing.clone().as_str()).as_str());
//             layers_string.push(spacing_front.clone() + prevs.join(spacing.clone().as_str()).as_str());
//             layers_string.push(spacing_front.clone() + parents.join(spacing.clone().as_str()).as_str());
//             layers_string.push(spacing_front.clone() + balance_factors.join(spacing.clone().as_str()).as_str());
//             layers_string.push(spacing_front.clone() + node_keys.join(spacing.clone().as_str()).as_str());
//             layers_string.push("".to_string());
//             half_spacing = spacing.clone();
//             spacing = spacing.clone() + spacing.clone().as_str() + "  ";
//         }
//
//         debug!("Tree:");
//         debug!("Vertical node arangement: Node, Value Balance factor, Parent, prev, next");
//         let print_string = "\n".to_string() + layers_string.iter().map(|s| s.as_str()).rev().collect::<Vec<_>>().join("\n").as_str();
//         debug!("{}", print_string);
//         debug!("depth: {}", max_depth);
//     }
}
