use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::hash::Hash;
use std::mem;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};

use scrypto::prelude::*;

pub struct ItemRef<'a, K: ScryptoSbor, V: ScryptoSbor> {
    item: KeyValueEntryRef<'a, Node<K, V>>,
}

impl<'a, K: ScryptoSbor, V: ScryptoSbor> Deref for ItemRef<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.item.value
    }
}

pub struct ItemRefMut<'a, K: ScryptoSbor, V: ScryptoSbor> {
    item: KeyValueEntryRefMut<'a, Node<K, V>>,
}

impl<'a, K: ScryptoSbor, V: ScryptoSbor> Deref for ItemRefMut<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.item.value
    }
}

impl<'a, K: ScryptoSbor, V: ScryptoSbor> DerefMut for ItemRefMut<'a, K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item.value
    }
}

#[derive(ScryptoSbor, Clone)]
pub(crate) struct Node<K: ScryptoSbor, V: ScryptoSbor> {
    pub(crate) key: K,
    pub(crate) value: V,
    pub(crate) left_child: Option<K>,
    pub(crate) right_child: Option<K>,
    pub(crate) parent: Option<K>,
    pub(crate) next: Option<K>,
    pub(crate) prev: Option<K>,
    pub(crate) balance_factor: i32,
}

impl<K: ScryptoSbor + Clone + Eq + Ord, V: ScryptoSbor> Node<K, V> {
    fn set_child(&mut self, direction: Direction, child: Option<K>) {
        match direction {
            Direction::Left => self.left_child = child,
            Direction::Right => self.right_child = child,
        }
    }
    fn replace_child(&mut self, old_child: &K, new_child: Option<K>) {
        if self.left_child == Some(old_child.clone()) {
            self.left_child = new_child
        } else if self.right_child == Some(old_child.clone()) {
            self.right_child = new_child;
        } else {
            panic!("Tried to over ride Node but was not a child");
        }
    }
    fn get_child(&self, direction: Direction) -> Option<K> {
        match direction {
            Direction::Left => self.left_child.clone(),
            Direction::Right => self.right_child.clone(),
        }
    }
    fn has_child(&self) -> bool {
        self.left_child.is_some() || self.right_child.is_some()
    }
    fn get_imbalance_direction(&self) -> Option<Direction> {
        Direction::from_balance_factor(self.balance_factor)
    }
    fn set_prev_next(&mut self, direction: Direction, node: Option<K>) {
        match direction {
            Direction::Left => self.prev = node,
            Direction::Right => self.next = node,
        }
    }
    fn get_prev_next(&self, direction: Direction) -> Option<K> {
        match direction {
            Direction::Left => self.prev.clone(),
            Direction::Right => self.next.clone(),
        }
    }
    fn direction_to_parent(&self) -> Option<Direction> {
        self.parent.as_ref().map(|parent| {
            Direction::from_ordering(parent.cmp(&self.key)).expect("Nodes should be unequal")
        })
    }
    fn direction_from_parent(&self) -> Option<Direction> {
        self.parent.as_ref().map(|parent| {
            Direction::from_ordering(self.key.cmp(parent)).expect("Nodes should be unequal")
        })
    }
    fn direction_from_other(&self, other: K) -> Option<Direction> {
        Some(Direction::from_ordering(other.cmp(&self.key)).expect("Nodes should be unequal"))
    }
    fn next(&self, direction: Direction) -> Option<K> {
        match direction {
            Direction::Left => self.prev.clone(),
            Direction::Right => self.next.clone(),
        }
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
    fn is_inside<K: Ord>(&self, value: &K, other: Bound<&K>) -> bool {
        match self {
            Self::Left => match other {
                Bound::Unbounded => true,
                Bound::Included(other) => value >= other,
                Bound::Excluded(other) => value > other,
            },
            Self::Right => match other {
                Bound::Unbounded => true,
                Bound::Included(other) => value <= other,
                Bound::Excluded(other) => value < other,
            }
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
            _ => None,
        }
    }
    fn from_ordering(ordering: Ordering) -> Option<Self> {
        match ordering {
            Less => Some(Self::Left),
            Greater => Some(Self::Right),
            Equal => None,
        }
    }
}

pub struct NodeIterator<'a, K: ScryptoSbor, V: ScryptoSbor, > {
    current: Option<K>,
    direction: Direction,
    end: Bound<K>,
    store: &'a KeyValueStore<K, Node<K, V>>,
}

impl<'a, K: ScryptoSbor + Clone + Ord + Eq + Display, V: ScryptoSbor + Clone> Iterator for NodeIterator<'a, K, V> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.store.get(&self.current.clone()?).expect("Node not found");
        let next = node.next(self.direction);
        self.current = match next.as_ref().map(|k| self.direction.is_inside(k, self.end.as_ref())) {
            Some(true) => next,
            _ => None,
        };
        Some(node.value.clone())
    }
}


pub struct NodeIteratorMut<'a, K: ScryptoSbor, V: ScryptoSbor> {
    current: Option<K>,
    direction: Direction,
    end: Bound<K>,
    store: &'a mut KeyValueStore<K, Node<K, V>>,
}

// This cannot be done without unsafe! we should not try it! If you want to try it yourself you need to add self.current in the struct (an optional KeyValueEntryRefMut<...>)
// impl<'a, K: ScryptoSbor + Clone + Ord + Eq + Display, V: ScryptoSbor + Clone> Iterator for NodeIteratorMut<'a, K, V> {
//     type Item = ItemMut<'a, K, V>;
// 
//     fn next(&mut self) -> Option<Self::Item> {
//         let node = self.store.get_mut(&self.current.clone()?).unwrap();
// 
//         let next = node.next(self.direction);
//         self.current = match next.as_ref().map(|k| self.direction.is_inside(k, self.end.as_ref())){
//             Some(true) => next,
//             _ => None,
//         };
//         unsafe {
//             Some(ItemMut{item:node})
//         }
//     }
// }

impl<'a, K: ScryptoSbor + Clone + Ord + Eq, V: ScryptoSbor + Clone> NodeIteratorMut<'a, K, V> {
    pub fn for_each(&mut self, mut function: impl FnMut(&mut V)) {
        while let Some(key) = self.current.clone() {
            let mut node = self.store.get_mut(&key).expect("Node not found");
            let next = node.next(self.direction);
            self.current = match next.as_ref().map(|k| self.direction.is_inside(k, self.end.as_ref())) {
                Some(true) => next,
                _ => None
            };
            let mut value = node.value.clone();
            function(&mut value);
            node.value = value;
        }
    }
}

/// A `AvlTree` is a balanced binary tree.
/// It is implemented as a double linked list with a binary tree on top.
/// The double linked list is used to iterate over the tree in order.
/// The binary tree is used to balance the tree.
/// The tree is balanced by keeping track of the balance factor of each node.
/// The balance factor is the height of the right subtree minus the height of the left subtree.
/// If the balance factor is greater than 1 or smaller than -1 the tree is unbalanced.
///
#[derive(ScryptoSbor)]
pub struct AvlTree<K: ScryptoSbor + Eq + Ord + Hash, V: ScryptoSbor> {
    pub(crate) root: Option<K>,
    store: KeyValueStore<K, Node<K, V>>,
    store_cache: HashMap<K, Node<K, ()>>,
}

impl<K: ScryptoSbor + Clone + Display + Eq + Ord + Hash + Debug, V: ScryptoSbor + Clone> AvlTree<K, V>
{
    pub fn new() -> Self {
        /// Creates a new empty `AvlTree`.
        AvlTree {
            root: None,
            store: KeyValueStore::new(),
            store_cache: HashMap::new(),
        }
    }

    /// Returns the value of the given key in a ItemRef.
    /// Usage:
    /// let tree = AvlTree::new();
    /// tree.insert(1, 1);
    /// let value = tree.get(&1).unwrap();
    /// assert_eq!(*value, 1);
    pub fn get(&self, key: &K) -> Option<ItemRef<K, V>> {
        self.store.get(key).map(|node| ItemRef { item: node })
    }

    /// Returns the value of the given key in a mutable wrapper, that writes back to the tree on drop.
    /// Usage:
    /// let tree = AvlTree::new();
    /// tree.insert(1, 1);
    /// {
    ///     let mut value = tree.get_mut(&1).unwrap();
    ///     *value = 2;
    /// }
    /// let value = tree.get(&1).unwrap();
    /// assert_eq!(*value, 2);
    pub fn get_mut(&mut self, key: &K) -> Option<ItemRefMut<K, V>> {
        self.store.get_mut(key).map(|n| ItemRefMut { item: n })
    }

    /// Return the internal representation of the tree, for the health checking.
    pub(crate) fn get_node(&mut self, key: &K) -> Option<Node<K, ()>> {
        self.cache_if_missing(key);
        // Carefully this is not synced with the store!
        self.store_cache.get(&key).map(|x| x.clone())
    }

    fn cache_if_missing(&mut self, key: &K) {
        if !self.store_cache.contains_key(&key) {
            self.store.get(&key).map(|data| {
                self.store_cache.insert(
                    key.clone(),
                    Node {
                        key: data.key.clone(),
                        value: (),
                        left_child: data.left_child.clone(),
                        right_child: data.right_child.clone(),
                        parent: data.parent.clone(),
                        prev: data.prev.clone(),
                        next: data.next.clone(),
                        balance_factor: data.balance_factor,
                    },
                )
            });
        }
    }

    fn flush_cache(&mut self) {
        for (key, value) in self.store_cache.iter() {
            let mut data = self.store.get_mut(key).expect("Node not found");
            data.left_child = value.left_child.clone();
            data.right_child = value.right_child.clone();
            data.parent = value.parent.clone();
            data.prev = value.prev.clone();
            data.next = value.next.clone();
            data.balance_factor = value.balance_factor.clone();
        }
        self.store_cache.clear();
    }
    fn get_mut_node(&mut self, key: &K) -> Option<&mut Node<K, ()>> {
        self.cache_if_missing(key);
        self.store_cache.get_mut(key)
    }

    /// Iterates backwards over the tree:
    /// Example:
    /// tree is initialized with all integers from 0 to 100.
    /// for i in tree.range_back(10..20) {
    ///   println!("{}", i);
    /// }
    /// gives:
    /// 19, 18, 17, 16, 15, 14, 13, 12, 11, 10
    /// The Include(start) and Exclude(end) can be changed
    /// For example:
    /// tree is initialized with all integers from 0 to 100.
    /// for i in tree.range_back((Excluded(10),Included(20))) {
    ///  println!("{}", i);
    /// }
    /// gives:
    /// 20, 19, 18, 17, 16, 15, 14, 13, 12, 11
    pub fn range_back<R>(&self, range: R) -> NodeIterator<K, V> where R: RangeBounds<K> {
        return self.range_internal(range.end_bound(), range.start_bound(), Direction::Left);
    }
    /// Iterate over the tree in order.
    /// Range is normally defined as Included(start) and Excluded(end).
    /// Example:
    /// tree is initialized with all integers from 0 to 100 and value = key.
    /// for i in tree.range(10..20) {
    ///    println!("{}", i);
    /// }
    /// gives:
    /// 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
    /// end can also be included either with Included(end) or:
    /// tree is initialized with all integers from 0 to 100, and value=key.
    /// for i in tree.range(10..=20) {
    ///   println!("{}", i);
    /// }
    /// gives:
    /// 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    pub fn range<R>(&self, range: R) -> NodeIterator<K, V> where R: RangeBounds<K> {
        return self.range_internal(range.start_bound(), range.end_bound(), Direction::Right);
    }
    /// Reversed mutable iterator that works only with for each:
    /// Example:
    /// tree is initialized with all integers from 0 to 100 and value = key.
    /// let mut idx = 0
    /// tree.range_back_mut(10..15).for_each(|x| {*x = idx; idx += 1;});
    /// for i in tree.range(0..30) {
    ///  println!("{}", i);
    /// }
    /// gives:
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 3, 2, 1, 0, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
    pub fn range_back_mut<R>(&mut self, range: R) -> NodeIteratorMut<K, V> where R: RangeBounds<K> {
        return self.range_mut_internal(range.end_bound(), range.start_bound(), Direction::Left);
    }

    /// Mutable iterator that works only with for each:
    /// Example:
    /// tree is initialized with all integers from 0 to 100 and value = key.
    /// let mut idx = 0
    /// tree.range_mut(10..20).for_each(|x| {*x = idx; idx += 1;} );
    /// for i in tree.range(0..30) {
    ///  println!("{}", i);
    /// }
    /// gives:
    /// Because the range is sorted after the keys.
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29
    pub fn range_mut<R>(&mut self, range: R) -> NodeIteratorMut<K, V> where R: RangeBounds<K> + Debug {
        return self.range_mut_internal(range.start_bound(), range.end_bound(), Direction::Right);
    }
    fn range_mut_internal(&mut self, start_bound: Bound<&K>, end_bound: Bound<&K>, direction: Direction) -> NodeIteratorMut<K, V> {
        let start = self.range_get_start(start_bound, direction);
        NodeIteratorMut {
            current: start,
            direction,
            end: end_bound.cloned(),
            store: &mut self.store,
        }
    }
    fn range_internal(&self, start_bound: Bound<&K>, end_bound: Bound<&K>, direction: Direction) -> NodeIterator<K, V> {
        let start = self.range_get_start(start_bound, direction);
        NodeIterator {
            current: start,
            direction,
            end: end_bound.cloned(),
            store: &self.store,
        }
    }

    fn range_get_start(&self, start_bound: Bound<&K>, direction: Direction) -> Option<K> {
        let start = match start_bound {
            Bound::Included(k) => self.store.get(k).map(|n| n.key.clone()),
            Bound::Excluded(k) => self.store.get(k).map(|n| n.next(direction)).flatten(),
            Bound::Unbounded => None,
        };

        let start = start.or_else(|| self.find_first_node(start_bound, direction));
        start
    }

    fn find_first_node(&self, lower_bound: Bound<&K>, direction: Direction) -> Option<K> {
        let mut current = self.root.clone();
        let mut result = None;
        while current.is_some() {
            let node = self.store.get(&current.clone().unwrap()).expect("Node of subtree should exist.");
            match direction.is_inside(&node.key, lower_bound) {
                true => {
                    current = node.get_child(direction).clone();
                }
                false => {
                    result = current.clone();
                    current = node.get_child(direction.opposite()).clone();
                }
            }
        }
        result
    }

    /// Inserts a new key value pair into the tree.
    /// Operation needs in the worst case 2*(log(n)+1) accesses to the KVStore.
    /// If the key already exists the old value is returned and the new value is inserted.
    /// Example:
    /// let tree = AvlTree::new();
    /// let old_value = tree.insert(1, 1);
    /// assert_eq!(old_value, None);
    /// let old_value = tree.insert(1, 2);
    /// assert_eq!(old_value, Some(1));
    /// let value = tree.get(&1).unwrap();
    /// assert_eq!(*value, 2);
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(mut existing_node) = self.store.get_mut(&key) {
            return Some(mem::replace(&mut existing_node.value, value));
        }
        let mut parent = self.insert_node_in_empty_spot(&key, value);
        let mut deepen = true;
        while let Some((node, insert_direction)) = parent {
            let mut cached_node = self.get_mut_node(&node).expect("Parent of insert should exist");
            parent = cached_node.parent.clone().zip(cached_node.direction_from_parent());
            if deepen {
                deepen = cached_node.balance_factor == 0;
                cached_node.balance_factor += insert_direction.direction_factor();
            }
            if cached_node.balance_factor.abs() == 2 {
                // shouldn't this break because we have the node cached and it gets change in balance?
                self.balance(&node, insert_direction);
            }
            if !deepen {
                break;
            }
        }
        self.flush_cache();
        None
    }

    fn insert_node_in_empty_spot(&mut self, key: &K, value: V) -> Option<(K, Direction)> {
        let mut node = self.root.clone().map(|x| self.get_node(&x).expect("Root should exist"));
        let mut parent_tuple = None;
        while let Some(current_node) = node {
            // get last element of parents
            let down_direction = Direction::from_ordering(key.cmp(&current_node.key));
            match down_direction {
                Some(down_direction) => {
                    parent_tuple = Some((current_node.key.clone(), down_direction));
                    node = current_node.get_child(down_direction).map(|n| self.get_node(&n).expect("Child should exist, because direction exists"));
                }
                None => {
                    // Key already exists do not balance parents just override value
                    panic!("Key already exists this should be caught in the beginning of insert");
                }
            }
        }
        match parent_tuple.as_ref() {
            Some((last, dir)) => {
                self.insert_node_and_adjust_pointers(last, key, value, *dir);
            }
            None => {
                // Tree is empty
                self.add_node(None, &key, value, None, None);
                self.root = Some(key.clone());
            }
        };
        parent_tuple
    }

    fn add_node(
        &mut self,
        parent: Option<K>,
        key: &K,
        value: V,
        prev: Option<K>,
        next: Option<K>,
    ) {
        self.store.insert(
            key.clone(),
            Node {
                key: key.clone(),
                value,
                left_child: None,
                right_child: None,
                next: next.clone(),
                prev: prev.clone(),
                parent: parent.clone(),
                balance_factor: 0,
            },
        );
        self.store_cache.insert(
            key.clone(),
            Node {
                key: key.clone(),
                value: (),
                left_child: None,
                right_child: None,
                next,
                prev,
                parent,
                balance_factor: 0,
            },
        );
    }

    fn insert_node_and_adjust_pointers(
        &mut self,
        parent_key: &K,
        key: &K,
        value: V,
        dir: Direction,
    ) {
        let other_neighbour: Option<K>;
        // one neighbour in the double linked list is always the parent and the other is the next or prev of the parent, depending on the direction.
        other_neighbour = self.get_node(parent_key).expect("Parent should exist").get_prev_next(dir);
        if let Some(neighbour_key) = other_neighbour.clone() {
            // Set the neighbour's prev_next to the new node.
            self.get_mut_node(&neighbour_key).expect("Neighbour should exist").set_prev_next(dir.opposite(), Some(key.clone()));
        }
        {
            // Set the parent's child to the new node.
            let parent = self.get_mut_node(parent_key).expect("Parent should exist");
            parent.set_prev_next(dir, Some(key.clone()));
            parent.set_child(dir, Some(key.clone()));
        }
        // Find the prev and next of the new node.
        let (prev, next) = match other_neighbour.clone() {
            Some(neighbour) => {
                (Some(parent_key.clone().min(neighbour.clone())), Some(parent_key.clone().max(neighbour)))
            }
            None => match dir {
                Direction::Left => {
                    (None, Some(parent_key.clone()))
                }
                Direction::Right => {
                    (Some(parent_key.clone()), None)
                }
            }
        };
        self.add_node(Some(parent_key.clone()), &key, value, prev, next);
    }

    /// Deletes the given key from the tree.
    /// Returns the value of the deleted key if it existed.
    /// Usage:
    /// let tree = AvlTree::new();
    /// tree.insert(1, 1);
    /// let value = tree.delete(1);
    /// assert_eq!(value, Some(1));
    /// let value = tree.delete(1);
    /// assert_eq!(value, None);
    /// let value = tree.get(&1);
    /// assert_eq!(value, None);
    pub fn delete(&mut self, key: K) -> Option<V> {
        // Remove mut if store can remove nodes.
        let del_node = self.get_node(&key);
        if del_node.is_none() {
            return None;
        }
        let (start_tuple, shortened) = self.rewire_tree_for_delete(del_node.unwrap());
        self.balance_tree_after_delete(start_tuple, shortened);
        self.flush_cache();
        self.store.remove(&key).map(|n| n.value)
    }

    fn balance_tree_after_delete(
        &mut self,
        mut node_tuple: Option<(K, Direction)>,
        mut shortened: bool,
    ) {
        while let Some((current_node, child_dir)) = node_tuple {
            if !shortened {
                break;
            }
            let parent_before_balance = self.get_node(&current_node).expect("Node should exist because key was saved earlier").parent;
            let (current_node_balance_factor, balance_child_direction) = {
                let mut current_node = self.get_mut_node(&current_node).expect("Node should exist because key was saved earlier");
                current_node.balance_factor += child_dir.direction_factor();
                // get balance direction before balancing because the parent can change afterwards.
                (current_node.balance_factor, current_node.direction_to_parent())
            };
            let mut new_root_balance_factor = None;

            if current_node_balance_factor.abs() == 2 {
                new_root_balance_factor = Some(self.balance(&current_node, child_dir.clone()));
            }
            // continue going up if bf is 0 after removing of child -> layer was removed!
            shortened = new_root_balance_factor.unwrap_or(current_node_balance_factor) == 0;
            node_tuple = parent_before_balance.zip(balance_child_direction);
        }
    }
    fn rewire_tree_for_delete(&mut self, del_node: Node<K, ()>) -> (Option<(K, Direction)>, bool) {
        let del_node_parent_tuple = del_node.parent.clone().zip(del_node.direction_to_parent());
        self.rewire_next_and_previous(&del_node);
        let replace_node = self.calculate_replace_node(&del_node);

        del_node.parent.as_ref().map(|parent|
            self.get_mut_node(&parent).expect("Parent not in KVStore").replace_child(&del_node.key, replace_node.clone())
        );
        let (replace_parent_tuple, shorten) = replace_node.clone()
            .map(|n| self.rewire_replace_node(&n, &del_node))
            .unzip();

        // Check if the root has to be replaced.
        if self.root == Some(del_node.key.clone()) {
            self.root = replace_node;
        }

        (replace_parent_tuple.or(del_node_parent_tuple), shorten.unwrap_or(true))
    }

    fn rewire_replace_node(
        &mut self,
        replace: &K,
        del_node: &Node<K, ()>,
    ) -> ((K, Direction), bool) {
        let (mut replace_parent_key, mut replace_parent_direction, non_empty_child) = self.replace_parent_and_children(replace, &del_node);
        let shorten;
        if del_node.key == replace_parent_key {
            // if parent is node to delete, we do not have to rewrite stuff because node will be lost anyway.
            // change balance factor of replace because will not be in the parent chain.
            let replace = self.get_mut_node(replace).expect("Replace should exist");
            let replace_balance_factor = del_node.balance_factor.clone() + replace.direction_from_other(del_node.key.clone()).expect("Should have different keys").direction_factor();
            replace.balance_factor = replace_balance_factor;
            shorten = replace_balance_factor == 0;
            del_node.parent.clone().map(|parent| {
                replace_parent_key = parent;
                replace_parent_direction = del_node.direction_to_parent().unwrap();
            });
        } else {
            self.delete_rewire_replace_parent(replace, &replace_parent_key, non_empty_child, del_node);
            shorten = true;
        }
        self.rewire_replace_child(del_node, replace);

        self.get_mut_node(replace).expect("Replace should exist").parent = del_node.parent.clone();
        ((replace_parent_key, replace_parent_direction), shorten)
    }

    fn replace_parent_and_children(&mut self, replace: &K, del_node: &Node<K, ()>) -> (K, Direction, Option<K>) {
        let replace = self.get_node(replace).expect("Node should exist.");
        let non_empty_child = replace.left_child.clone().or(replace.right_child.clone());
        // rewire possible child of replace if replace and del_node are not parent and child.
        if replace.parent.as_ref() != Some(&del_node.key) {
            non_empty_child.as_ref().map(|k| self.get_mut_node(k).expect("Replace child not in store but present in replace as child").parent = replace.parent.clone());
        }
        let replace_parent_key = replace.parent.clone().expect("should have parent because it is a child of the del_node.");
        (replace_parent_key, replace.direction_to_parent().unwrap(), non_empty_child)
    }
    fn rewire_next_and_previous(&mut self, del_node: &Node<K, ()>) {
        // Jump over del_node in next and previous.
        del_node.next.as_ref().map(|next| self.get_mut_node(next).expect("Next is not in store").prev = del_node.prev.clone());
        del_node.prev.as_ref().map(|prev| self.get_mut_node(prev).expect("Del node prev is not in store").next = del_node.next.clone());
    }
    fn rewire_replace_child(&mut self, del_node: &Node<K, ()>, replace: &K) {
        let children: Vec<(K, Direction)> = [Direction::Left, Direction::Right].into_iter()
            .map(|d| del_node.get_child(d).zip(Some(d)))
            .filter(|k| k.is_some()).map(|k| k.unwrap())
            .filter(|(k, _)| k != replace).collect();
        if children.len() == 0 {
            return;
        }
        children.iter().for_each(|(child, _)| {
            self.get_mut_node(child).expect("Child of delete not in store but in tree").parent = Some(replace.clone());
        });
        {
            let replace_node = self.get_mut_node(replace).expect("Replace should exist");
            children.into_iter().for_each(|(child, direction)| {
                replace_node.set_child(direction, Some(child));
            });
        }
    }
    fn delete_rewire_replace_parent(
        &mut self,
        replace: &K,
        replace_parent_key: &K,
        non_empty_child: Option<K>,
        del_node: &Node<K, ()>,
    ) {
        let direction = self.get_node(replace).expect("Node should exist").direction_to_parent().expect("Should have parent");
        let replace_parent = self.get_mut_node(replace_parent_key).expect("Replace parent should exist");
        replace_parent.replace_child(replace, non_empty_child.clone());
        replace_parent.set_child(direction.opposite(), non_empty_child);
        // replace should max have one child so we have to rewire the leftover child:
        self.get_mut_node(replace).expect("Replace should exist").balance_factor = del_node.balance_factor;
    }
    fn calculate_replace_node(&mut self, del_node: &Node<K, ()>) -> Option<K> {
        if !del_node.has_child() {
            return None;
        }
        let imbalance_direction = del_node.get_imbalance_direction();
        let imbalance_next = imbalance_direction.map(|d| del_node.get_prev_next(d).unwrap());
        let replace_key = imbalance_next.or_else(|| del_node.next.clone().map_or(del_node.prev.clone(), |n| Some(n)));
        replace_key
    }
    fn balance(&mut self, root: &K, balance_direction: Direction) -> i32 {
        let child_id = self.get_node(root).expect("Node should exist").get_child(balance_direction).expect("Child should exist");
        let child_balance_factor = self.get_node(&child_id).expect("Node should exist").balance_factor;
        if child_balance_factor.signum() == balance_direction.direction_factor() {
            self.balance_with_subtree_in_same_direction(root, &child_id, balance_direction)
        } else if child_balance_factor == 0 {
            self.balance_with_zero_bf_subtree(root, &child_id, balance_direction)
        } else {
            self.balance_with_subtree_in_different_direction(root, &child_id, balance_direction)
        }
    }

    fn balance_with_subtree_in_same_direction(
        &mut self,
        root: &K,
        child: &K,
        imbalance_direction: Direction,
    ) -> i32 {
        /*
         *  Before Balance:
         *      R
         *    /  \
         *   A    C
         *       / \
         *      A   L
         *         / \
         *        A   A
         *
         *  After Balance
         *     C
         *    / \
         *   R   L
         *  / \ / \
         * A  A A  A
         */
        self.get_mut_node(child).expect("Child in balance should exist").balance_factor = 0;

        self.get_mut_node(root).expect("Balance root should exist").balance_factor = 0;
        self.rotate(imbalance_direction.opposite(), root, child);
        // Balance_factor of new root=child=0
        0
    }
    fn balance_with_zero_bf_subtree(
        &mut self,
        root: &K,
        child: &K,
        imbalance_direction: Direction,
    ) -> i32 {
        /*
         * imbalance direction = right
         *  Before Balance :
         *   B.depth + 1 == A.depth -> C.bf = 0
         *
         *      R
         *    /  \
         *   A    C
         *       / \
         *      B   L
         *         / \
         *        A   A
         *
         *  After Balance
         *     C
         *    / \
         *   R   L
         *  / \ / \
         * A  B A  A
         * -> C.bf = -1, R.bf = -1
         *
         */
        self.get_mut_node(root).expect("Root in balance should exist").balance_factor = imbalance_direction.direction_factor();
        self.get_mut_node(child).expect("Child in balance should exist").balance_factor = imbalance_direction.opposite().direction_factor();
        self.rotate(imbalance_direction.opposite(), root, child);
        // Balance_factor of new root=child
        imbalance_direction.opposite().direction_factor()
    }
    fn balance_with_subtree_in_different_direction(
        &mut self,
        root: &K,
        child: &K,
        imbalance_direction: Direction,
    ) -> i32 {
        /*
         * imbalance direction = right
         *  Before Balance :
         *
         *
         *      R
         *    /  \
         *   A    C
         *       / \
         *      NR  A
         *     / \
         *    B   C
         *
         *  After Balance
         *     NR
         *    / \
         *   R   C
         *  / \ / \
         * A  B C  A
         * -> Balance factor of R and C depend on old balance factor of NR
         */
        // This reference is not synced with the parents list. However, this child node should be further down in the tree and not in the parents list
        let new_root = self.get_node(child).expect("Child of balance should exist").get_child(imbalance_direction.opposite()).unwrap();
        let new_root_balance_factor = self.get_node(&new_root).expect("New root should also exist else we would not be in this case!").balance_factor;
        {
            let root = self.get_mut_node(root).expect("Root in balance should exist");
            root.balance_factor = 0;
            // If new root was balanced in the same direction, root has a child more
            if new_root_balance_factor == imbalance_direction.direction_factor() {
                root.balance_factor = imbalance_direction.opposite().direction_factor();
            }
        }
        {
            let child = self.get_mut_node(child).expect("Child in balance should exist");
            child.balance_factor = 0;
            // If new root was balanced in the same direction, child has a child more
            if new_root_balance_factor == imbalance_direction.opposite().direction_factor() {
                child.balance_factor = imbalance_direction.direction_factor();
            }
        }
        self.rotate(imbalance_direction, child, &new_root);
        self.rotate(imbalance_direction.opposite(), root, &new_root);
        // reset new_root balance factor here because the old value is needed before
        self.get_mut_node(&new_root).expect("New root should still exist").balance_factor = 0;
        // balance_factor_of_new_root
        0
    }

    fn rotate(&mut self, rotate_direction: Direction, root: &K, child: &K) {
        /*
           *  Rotate left:
           *      R
           *    /  \
           *   _    C
           *       / \
           *      RL  _
           *
           * to:
           *     C
           *    / \
           *   R   _
           *  / \
           * _   RL
           or
           *  Rotate right:
           *      R
           *    /  \
           *   C    _
           *  / \
           * _   lr
           to:
           *     C
           *    / \
           *   _   R
           *      / \
           *     LR  _
        */
        let parent_key = self.rotate_rewire_parent(root, child);
        if parent_key.is_none() {
            self.root = Some(child.clone());
        }
        let left_over_child;
        {
            let child = self.get_mut_node(child).expect("Rotate without child at right position");
            child.parent = parent_key;
            left_over_child = child.get_child(rotate_direction);
            child.set_child(rotate_direction, Some(root.clone()));
        }
        if let Some(old_root_child_key) = left_over_child.as_ref() {
            self.get_mut_node(old_root_child_key).expect("Child of child not in store").parent = Some(root.clone());
        }
        let root = self.get_mut_node(root).expect("Rotate without root in Store");
        root.set_child(rotate_direction.opposite(), left_over_child);
        root.parent = Some(child.clone());
    }
    fn rotate_rewire_parent(&mut self, root: &K, child: &K) -> Option<K> {
        let parent_id = self.get_node(root).expect("rotate root does not exist").parent;
        parent_id.as_ref().map(|parent_id| {
            self.get_mut_node(parent_id).expect("Parent of rotate root not in store").replace_child(root, Some(child.clone()))
        });
        parent_id
    }
}
