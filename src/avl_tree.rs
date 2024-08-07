use scrypto::prelude::*;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::hash::Hash;
use std::mem;
use std::ops::{Bound, Deref, DerefMut, RangeBounds};

/// An `AvlTree` is a balanced binary tree.
/// It is implemented as a double linked list with a binary tree on top.
/// The double linked list is used to iterate over the tree in order.
/// The binary tree is used to balance the tree.
/// The tree is balanced by keeping track of the balance factor of each node.
/// The balance factor is the height of the right subtree minus the height of the left subtree.
/// If the balance factor is greater than 1 or smaller than -1 the tree is unbalanced.
///
#[derive(ScryptoSbor)]
pub struct AvlTree<K: ScryptoSbor + Eq + Ord + Hash, V: ScryptoSbor> {
    /// The root of the tree.
    pub(crate) root: Option<K>,
    /// The store of the tree, the node stores the key, value, and navigation pointers in the tree, they are more explained in the Node struct.
    store: KeyValueStore<K, Node<K, V>>,
    /// Cache the node information without the value.
    store_cache: HashMap<K, Node<K, ()>>,
}

impl<K: ScryptoSbor + Clone + Display + Eq + Ord + Hash + Debug, V: ScryptoSbor + Clone> Default
    for AvlTree<K, V>
{
    fn default() -> Self {
        AvlTree::new()
    }
}

impl<K: ScryptoSbor + Clone + Display + Eq + Ord + Hash + Debug, V: ScryptoSbor + Clone>
    AvlTree<K, V>
{
    /// Creates an empty `AvlTree`.
    pub fn new() -> Self {
        AvlTree {
            root: None,
            store: KeyValueStore::new(),
            store_cache: HashMap::new(),
        }
    }

    /// Returns the value of the given key in a ItemRef.
    /// ```
    /// let tree = AvlTree::new();
    /// tree.insert(1, 1);
    /// let value = tree.get(&1).unwrap();
    /// assert_eq!(*value, 1);
    /// ```
    pub fn get(&self, key: &K) -> Option<ItemRef<K, V>> {
        self.store.get(key).map(|node| ItemRef { item: node })
    }

    /// Returns the value of the given key in a mutable wrapper, that writes back to the tree on drop.
    /// ```
    /// let tree = AvlTree::new();
    /// tree.insert(1, 1);
    /// {
    ///     let mut value = tree.get_mut(&1).unwrap();
    ///     *value = 2;
    /// }
    /// let value = tree.get(&1).unwrap();
    /// assert_eq!(*value, 2);
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<ItemRefMut<K, V>> {
        self.store.get_mut(key).map(|n| ItemRefMut { item: n })
    }

    /// Inserts a new key value pair into the tree.
    /// Operation needs in the worst case `2*(log(n)+1)` accesses to the KVStore.
    ///
    /// If the key already exists the old value is returned and the new value is inserted.
    ///
    /// Example:
    /// ```
    /// let tree = AvlTree::new();
    /// let old_value = tree.insert(1, 1);
    /// assert_eq!(old_value, None);
    /// let old_value = tree.insert(1, 2);
    /// assert_eq!(old_value, Some(1));
    /// let value = tree.get(&1).unwrap();
    /// assert_eq!(*value, 2);
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(mut existing_node) = self.store.get_mut(&key) {
            return Some(mem::replace(&mut existing_node.value, value));
        }
        let parent = self.insert_node_in_empty_spot(&key, value);
        self.balance_after_insert(parent);
        self.flush_cache();
        None
    }

    /// Deletes the given key from the tree.
    /// Returns the value of the deleted key if it existed.
    /// ```
    /// let tree = AvlTree::new();
    /// tree.insert(1, 1);
    /// let value = tree.remove(1);
    /// assert_eq!(value, Some(1));
    /// let value = tree.remove(1);
    /// assert_eq!(value, None);
    /// let value = tree.get(&1);
    /// assert_eq!(value, None);
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if !self.contains_key(key) {
            return None;
        }
        let (start_tuple, shortened) = self.rewire_tree_for_delete(key);
        self.balance_tree_after_delete(start_tuple, shortened);
        self.flush_cache();
        self.store.remove(&key).map(|n| n.value)
    }

    /// Iterate over the tree values in order of the keys.
    /// Range is normally defined as Included(start) and Excluded(end).
    ///
    /// Example:
    ///
    /// Tree is initialized with all integers from 0 to 100 and value = key.
    /// ```
    /// for (k: K, v: V, next_key: Option<K>) in tree.range(10..20) {
    ///     println!("{}", k);
    /// }
    /// ```
    ///
    /// Gives:
    /// ```
    /// 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
    /// ```
    ///
    /// The end can also be included either with Included(end) or:
    /// ```rust
    /// for (k: K, v: V, next_key: Option<K>) in tree.range(10..=20) {
    ///     println!("{}", k);
    /// }
    /// ```
    ///
    /// Gives:
    /// ```
    /// 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    /// ```
    /// The NodeIterator can also be called with for_each or for_each_node
    pub fn range<R: RangeBounds<K>>(&self, range: R) -> NodeIterator<K, V> {
        return self.range_internal(range.start_bound(), range.end_bound(), Direction::Right);
    }

    /// Iterates backwards over the tree values.
    ///
    /// Example:
    ///
    /// Tree is initialized with all integers from 0 to 100.
    /// ```
    /// for (k: K, v: V, next_key: Option<K>) in tree.range_back(10..=20) {
    ///     println!("{}", k);
    /// }
    /// ```
    ///
    /// Gives:
    /// ```
    /// 19, 18, 17, 16, 15, 14, 13, 12, 11, 10
    /// ```
    ///
    /// The Include(start) and Exclude(end) can be changed:
    ///
    /// E.g. tree is initialized with all integers from 0 to 100.
    /// ```rust
    /// for (k: K, v: V, next_key: Option<K>) in tree.range_back(Excluded(10),Included(20)) {
    ///     println!("{}", k);
    /// }
    /// ````
    ///
    /// Gives:
    /// ```
    /// 20, 19, 18, 17, 16, 15, 14, 13, 12, 11
    /// ```
    pub fn range_back<R: RangeBounds<K>>(&self, range: R) -> NodeIterator<K, V> {
        return self.range_internal(range.end_bound(), range.start_bound(), Direction::Left);
    }

    /// Mutable iterator over the values that works only with for each.
    ///
    /// Example:
    ///
    /// Tree is initialized with all integers from 0 to 100 and value = key.
    /// ```
    /// let mut idx = 0
    /// tree.range_mut(10..20).for_each(|x| {*x = idx; idx += 1;} );
    /// for (k, _, _) in tree.range(0..30) {
    ///     println!("{}", k);
    /// }
    /// ```
    ///
    /// Gives:
    /// Because the range is sorted after the keys.
    /// ```
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29
    /// ```
    pub fn range_mut<R: RangeBounds<K>>(&mut self, range: R) -> NodeIteratorMut<K, V> {
        return self.range_mut_internal(range.start_bound(), range.end_bound(), Direction::Right);
    }

    /// Reversed mutable iterator over the values that works only with for each.
    ///
    /// Example:
    ///
    /// Tree is initialized with all integers from 0 to 100 and value = key.
    /// ```
    /// let mut idx = 0;
    /// tree.range_back_mut(10..15).for_each(|x| {*x = idx; idx += 1;});
    /// for (k, _, _) in tree.range(0..30) {
    ///     println!("{}", k);
    /// }
    /// ```
    ///
    /// Gives:
    /// ```
    /// 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 4, 3, 2, 1, 0, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25
    /// ```
    pub fn range_back_mut<R: RangeBounds<K>>(&mut self, range: R) -> NodeIteratorMut<K, V> {
        return self.range_mut_internal(range.end_bound(), range.start_bound(), Direction::Left);
    }

    // PRIVATE METHODS

    /// Return the internal representation of the tree, public in crate for the health checking.
    pub(crate) fn get_node(&mut self, key: &K) -> Option<&Node<K, ()>> {
        self.cache_if_missing(key);
        // Carefully this is not synced with the store!
        self.store_cache.get(&key)
    }

    /// Return the internal representation of the tree.
    fn get_mut_node(&mut self, key: &K) -> Option<&mut Node<K, ()>> {
        self.cache_if_missing(key);
        self.store_cache.get_mut(key)
    }

    /// Caches the node information from the radix KV store.
    fn cache_if_missing(&mut self, key: &K) {
        if self.store_cache.contains_key(&key) {
            return;
        }
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

    ///  Check if key is present in the tree.
    fn contains_key(&mut self, key: &K) -> bool {
        self.cache_if_missing(key);
        self.store_cache.contains_key(key)
    }

    /// empties the cache and writes back the changes to the radix KV store.
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

    /// Wrapper function for range and range_back.
    fn range_internal(
        &self,
        start_bound: Bound<&K>,
        end_bound: Bound<&K>,
        direction: Direction,
    ) -> NodeIterator<K, V> {
        let start = self.range_get_start(start_bound, end_bound, direction);
        NodeIterator {
            current: start,
            direction,
            end: end_bound.cloned(),
            store: &self.store,
        }
    }

    /// Wrapper function for range_mut and range_back_mut.
    fn range_mut_internal(
        &mut self,
        start_bound: Bound<&K>,
        end_bound: Bound<&K>,
        direction: Direction,
    ) -> NodeIteratorMut<K, V> {
        let start = self.range_get_start(start_bound, end_bound, direction);
        NodeIteratorMut {
            current: start,
            direction,
            end: end_bound.cloned(),
            store: &mut self.store,
        }
    }

    /// Get the first node that is inside the range. If the bound is in the tree O(1), otherwise O(log n).
    /// Parameters:
    /// - start_bound: The start bound of the range.
    /// - end_bound: The end bound of the range.
    /// - direction: The direction of the iterator.
    ///
    /// Returns:
    /// - Some(K): The key of the first node that is inside the range.
    fn range_get_start(
        &self,
        start_bound: Bound<&K>,
        end_bound: Bound<&K>,
        direction: Direction,
    ) -> Option<K> {
        // Get starting node, if it is inside the store we can derive the start in O(1).
        // If self.store.get(k) is Some, the bound is contained inside the store. So the start is either k or the next node.
        let start: Option<Option<K>> = match start_bound {
            Bound::Included(k) => self.store.get(k).map(|n| Some(n.key.clone())),
            Bound::Excluded(k) => self.store.get(k).map(|n| n.next(direction)),
            Bound::Unbounded => None,
        };

        // When start is None we could not find the start bound directly in the store and we have to search in
        // the tree with find_first_node.
        // Afterwards we check if the starting node is inside the range.
        start
            .unwrap_or_else(|| self.find_first_node(start_bound, direction))
            .filter(|s| end_bound.within_bound(&s, direction))
    }

    /// Finds the initial node within the specified range based on the given direction.
    /// Iteratively traverses the tree and returns the most left or right node in the tree within the range.
    /// The direction parameter determines if it is left or right.
    fn find_first_node(&self, start_bound: Bound<&K>, iterator_direction: Direction) -> Option<K> {
        let mut current = self.root.clone();
        let mut result = None;
        while current.is_some() {
            let node = self
                .store
                .get(&current.clone().unwrap())
                .expect("Node of subtree should exist.");
            match start_bound.within_bound(&node.key, iterator_direction.opposite()) {
                true => {
                    result = current.clone();
                    // Current node is inside the range -> go to the boarder of the range
                    current = node.get_child(iterator_direction.opposite()).clone();
                }
                false => {
                    // Current node is outside the range -> go towards the range.
                    current = node.get_child(iterator_direction).clone();
                }
            }
        }
        result
    }

    /// Inserts a new key value pair into the tree.
    ///
    /// This function searches for an appropriate position for the key-value pair
    /// in the tree. If the tree is empty, the key-value pair becomes the root.
    /// Otherwise, it's inserted as a child of an existing node.
    /// If the key already exists in the tree, the function panics.
    ///
    /// Returns:
    /// - Some((K, Direction)): When the key-value pair is added to the tree,
    ///   returning the key of the parent node and the direction where the new
    ///   node was inserted.
    /// - None: When the tree is empty and the key-value pair becomes the root.
    ///
    fn insert_node_in_empty_spot(&mut self, key: &K, value: V) -> Option<(K, Direction)> {
        let mut current = self.root.clone();
        let mut parent = None;
        while let Some(parent_key) = current.as_ref() {
            let current_node = self.get_node(parent_key).expect("Root should exist");
            parent = current;
            current = current_node
                .get_child_in_key_direction(key)
                .expect("Parent should have child in key direction")
                .cloned();
        }
        match parent {
            Some(parent_key) => {
                let dir = Direction::from_ordering(key.cmp(&parent_key))
                    .expect("Parent has to be different");
                self.insert_node_and_adjust_pointers(&parent_key, key, value, dir);
                Some((parent_key, dir))
            }
            None => {
                // Tree is empty
                self.add_node(None, &key, value, None, None);
                self.root = Some(key.clone());
                None
            }
        }
    }

    /// Balance tree after inserting a node
    /// This function goes up the tree from the inserted node and balances a level if it is
    /// necessary.
    ///
    /// parent_info: Tuple of the node above inserted node and direction of parent
    fn balance_after_insert(&mut self, mut parent_info: Option<(K, Direction)>) {
        let mut deepen = true;
        while deepen && parent_info.is_some() {
            let (node, insert_direction) = parent_info.unwrap();
            let cached_node = self
                .get_mut_node(&node)
                .expect("Parent of insert should exist");
            parent_info = cached_node
                .parent
                .clone()
                .zip(cached_node.direction_to_parent().map(|d| d.opposite()));
            if deepen {
                deepen = cached_node.balance_factor == 0;
                cached_node.balance_factor += insert_direction.direction_factor();
            }
            if cached_node.balance_factor.abs() == 2 {
                self.balance(&node, insert_direction);
            }
            if !deepen {
                break;
            }
        }
    }

    /// Adds a new node to the primary store and a reference entry to the cache.
    fn add_node(&mut self, parent: Option<K>, key: &K, value: V, prev: Option<K>, next: Option<K>) {
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

    /// Inserts a node into the tree and adjusts the surrounding node pointers accordingly.
    ///
    /// This function inserts a new node as a child of the specified parent in the given direction (`dir`).
    /// It also adjusts the navigation pointers (i.e., `prev` and `next`) of the neighboring nodes
    /// to maintain the integrity of the doubly-linked list structure.
    ///
    fn insert_node_and_adjust_pointers(
        &mut self,
        parent_key: &K,
        key: &K,
        value: V,
        dir: Direction,
    ) {
        // one neighbour in the double linked list is always the parent and the other is the next or prev of the parent, depending on the direction.
        let other_neighbour = self
            .get_node(parent_key)
            .expect("Parent should exist")
            .get_prev_next(dir);
        // If the other neighbour exists, update its pointer to the new node.
        if let Some(neighbour_key) = other_neighbour.clone() {
            let neighbour = self
                .get_mut_node(&neighbour_key)
                .expect("Neighbour should exist");
            neighbour.set_prev_next(dir.opposite(), Some(key.clone()));
        }

        // Set the parent's child to the new node.
        let parent = self.get_mut_node(parent_key).expect("Parent should exist");
        parent.set_prev_next(dir, Some(key.clone()));
        parent.set_child(dir, Some(key.clone()));

        // Find the prev and next of the new node.
        let (prev, next) = match other_neighbour.clone() {
            Some(neighbour) => {
                let min_key = parent_key.clone().min(neighbour.clone());
                let max_key = parent_key.clone().max(neighbour);
                (Some(min_key), Some(max_key))
            }
            None => match dir {
                Direction::Left => (None, Some(parent_key.clone())),
                Direction::Right => (Some(parent_key.clone()), None),
            },
        };
        self.add_node(Some(parent_key.clone()), &key, value, prev, next);
    }

    /// Balances the tree following a node deletion.
    ///
    /// After a node is deleted from the tree, the balance factor of the ancestors may be affected.
    /// This method traverses up the tree from the deletion/replacement parent, adjusting the balance factors
    /// and potentially performing rotations to ensure that the AVL tree properties are maintained.
    ///
    /// The method operates iteratively, starting from a specified node and moving upwards towards
    /// the root. Balancing is performed based on the provided direction (`Direction`), indicating
    /// the side where the deletion took place.
    ///
    /// # Parameters
    /// - `node_tuple`: Contains the starting node key and the direction of the child that was deleted
    /// or where the tree got shortened. If `None`, the balancing procedure is not initiated.
    /// - `shortened`: Indicates if the height of the subtree has decreased as a result of the deletion.
    /// Balancing will continue upwards until this is `false`.
    fn balance_tree_after_delete(
        &mut self,
        mut node_tuple: Option<(K, Direction)>,
        mut shortened: bool,
    ) {
        while let Some((current_node, child_dir)) = node_tuple {
            if !shortened {
                break;
            }
            let parent_before_balance = self
                .get_node(&current_node)
                .cloned()
                .expect("Node should exist because key was saved earlier")
                .parent;
            let (current_node_balance_factor, balance_child_direction) = {
                let current_node = self
                    .get_mut_node(&current_node)
                    .expect("Node should exist because key was saved earlier");
                current_node.balance_factor += child_dir.direction_factor();
                // get balance direction before balancing because the parent can change afterwards.
                (
                    current_node.balance_factor,
                    current_node.direction_to_parent(),
                )
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

    /// Adjusts the tree structure after a node deletion.
    /// When deleting a node from a binary tree, it's possible that the tree structure
    /// will need to be modified to maintain the binary search property. This function
    /// handles these adjustments by ensuring that the links (or "wires") between nodes
    /// are correctly set after the deletion.
    ///
    /// # Arguments
    ///
    /// * `del_node_key` - The key of the node that is going to be deleted.
    ///
    /// # Returns
    ///
    /// * A tuple consisting of:
    ///     * An `Option` containing a tuple with the key of the parent node and its
    ///       direction (`Left` or `Right`) relative to its parent. The parent node is either:
    ///       the parent of delete node, or the replacement node (if delete node was replaced).
    ///       `None` if the node had no parent (i.e., it was the root).
    ///     * A boolean indicating if the subtree was shortened as a result of the deletion.
    fn rewire_tree_for_delete(&mut self, del_node_key: &K) -> (Option<(K, Direction)>, bool) {
        let del_node = self
            .get_node(del_node_key)
            .expect("Node should be present, because this gets checked in the beginning of delete.")
            .clone();
        let del_node_parent_tuple = del_node.parent.clone().zip(del_node.direction_to_parent());
        // rewire next and previous (if there is a replace node it is either next or previous so this works out without information about the replace node)
        self.rewire_next_and_previous(&del_node);
        let replace_node = self.calculate_replace_node(&del_node);

        let (replace_parent_tuple, shorten) = match replace_node.clone() {
            Some(node) => Some(self.rewire_replace_node(&node, &del_node)).unzip(),
            None => (None, Some(true)),
        };
        self.replace_del_node_in_parent(&del_node, replace_node.clone());

        // Check if the root has to be replaced.
        if self.root == Some(del_node.key.clone()) {
            self.root = replace_node;
        }

        (
            replace_parent_tuple.or(del_node_parent_tuple),
            shorten.unwrap_or(true),
        )
    }

    /// Given a node set for deletion (`del_node`), this function calculates
    /// which node (if any) should replace the node being deleted in the AVL tree.
    ///
    /// Returns:
    /// - `None` if the node has no children.
    /// - `Some(K)` where `K` is the key of the replacement node.
    fn calculate_replace_node(&mut self, del_node: &Node<K, ()>) -> Option<K> {
        if !del_node.has_child() {
            return None;
        }
        // Get the direction of imbalance (if it exists).
        let imbalance_direction = del_node.get_imbalance_direction();

        // Based on imbalance direction, find the next node.
        let imbalance_next = imbalance_direction.and_then(|d| del_node.get_prev_next(d));
        // If there is no imbalance direction, then take any of prev or next.
        let replace_key = imbalance_next.or(del_node.next.clone().or(del_node.prev.clone()));
        replace_key
    }

    /// Replaces a node set for deletion (`del_node`) in its parent's children
    /// with the given replacement node key (`replace_node`).
    ///
    /// If `replace_node` is `None`, it effectively removes `del_node` from its parent's children.
    fn replace_del_node_in_parent(&mut self, del_node: &Node<K, ()>, replace_node: Option<K>) {
        if let Some(parent_key) = &del_node.parent {
            let parent_node = self
                .get_mut_node(&parent_key)
                .expect("Parent not in KVStore");
            parent_node.replace_child(&del_node.key, replace_node);
        }
    }

    /// Remove delete node from double linked list.
    /// Does not need information about the replacement node, because it is either next or previous.
    /// So it will be correctly linked after this function.
    fn rewire_next_and_previous(&mut self, del_node: &Node<K, ()>) {
        // Jump over del_node in next and previous.
        del_node.next.as_ref().map(|next| {
            self.get_mut_node(next).expect("Next is not in store").prev = del_node.prev.clone();
        });
        del_node.prev.as_ref().map(|prev| {
            self.get_mut_node(prev)
                .expect("Del node prev is not in store")
                .next = del_node.next.clone();
        });
    }

    /// Reconfigures the tree structure after a node deletion, focusing on the replacement node.
    ///
    /// When a node is deleted, and a replacement node is selected to take its place,
    /// this function ensures that all the tree and double linked list links are correctly updated.
    /// This might involve changing the parent of the replacement node, adjusting balance
    /// factors, or updating child pointers.
    ///
    /// # Arguments
    ///
    /// * `replace` - The key of the node that will replace the deleted node.
    /// * `del_node` - The node being deleted.
    ///
    /// # Returns
    ///
    /// * A tuple consisting of:
    ///     * A tuple with the key of the parent node of the replacement node and its
    ///       direction (`Left` or `Right`) relative to its parent. This provides
    ///       context about which side of the parent the replacement node was on.
    ///     * A boolean indicating if the subtree was shortened as a result of the re-wiring.
    fn rewire_replace_node(
        &mut self,
        replace: &K,
        del_node: &Node<K, ()>,
    ) -> ((K, Direction), bool) {
        let replace = self.get_node(replace).expect("Node should exist.").clone();
        let replace_child = self.rewire_replace_node_children(&replace, del_node);
        let replace_parent_information =
            self.rewire_replace_node_parent(&replace, &del_node, replace_child);
        self.rewire_delete_node_child(del_node, &replace.key);
        self.get_mut_node(&replace.key)
            .expect("Replace should exist")
            .parent = del_node.parent.clone();
        replace_parent_information
    }

    /// Rewires the children of the replacement node when deleting a node from the tree.
    ///
    /// If the replacement node (`replace`) has children, this function will update
    /// the parent of the replacement node's child.
    /// This only happens if replacement node and the node to be deleted (`del_node`) are not parent and child.
    ///
    /// # Arguments
    /// * `replace`: The node that is chosen as the replacement during deletion.
    /// * `del_node`: The node that is being deleted from the tree.
    ///
    /// # Returns
    /// Returns an `Option<K>` that contains the key of the child of the replacement node if it exists; otherwise, returns `None`.
    fn rewire_replace_node_children(
        &mut self,
        replace: &Node<K, ()>,
        del_node: &Node<K, ()>,
    ) -> Option<K> {
        let replace_child = replace.left_child.clone().or(replace.right_child.clone());
        // rewire possible child of replace if replace and del_node are not parent and child.
        if replace.parent.as_ref() != Some(&del_node.key) {
            replace_child.clone().map(|k| {
                self.get_mut_node(&k)
                    .expect("Replace child not in store but present in replace as child")
                    .parent = replace.parent.clone();
            });
        }
        replace_child
    }

    /// Rewires the parent of the replacement node after deleting a node from the tree.
    ///
    /// This function handles the necessary updates to the parent of the `replace` node,
    /// IF the `replace` node is the child of the `del_node` node, the parent does not need to be changed,
    /// because the node to delete will not be in the tree afterwards.
    /// In this case the balance factor of replace needs to be updated immediately because it is not in the chain of parents.
    ///
    /// # Arguments
    /// * `replace`: The node that is chosen as the replacement during deletion.
    /// * `del_node`: The node that is being deleted from the tree.
    /// * `replace_child`: The child node of the `replace` node, if it exists.
    ///
    /// # Returns
    /// Returns a tuple consisting of:
    /// * A tuple `(K, Direction)` indicating the key of the replacement node's parent and the direction of the replacement node with respect to its parent.
    /// * A `bool` flag indicating whether the tree was shortened as a result of the rewire operation.
    fn rewire_replace_node_parent(
        &mut self,
        replace: &Node<K, ()>,
        del_node: &Node<K, ()>,
        replace_child: Option<K>,
    ) -> ((K, Direction), bool) {
        let mut replace_parent_key = replace
            .parent
            .clone()
            .expect("should have parent because it is a child of the del_node.");
        let mut replace_parent_direction = replace
            .direction_to_parent()
            .expect("should have parent because it is a child of the del_node.");
        let shorten;
        if del_node.key == replace_parent_key {
            // if parent is node to delete, we do not have to rewrite stuff because node will be lost anyway.
            // change balance factor of replace because it will not be in the parent chain.
            let replace = self
                .get_mut_node(&replace.key)
                .expect("Replace should exist");
            let replace_balance_factor = del_node.balance_factor.clone()
                + replace
                    .direction_from_other(del_node.key.clone())
                    .expect("Should have different keys")
                    .direction_factor();
            replace.balance_factor = replace_balance_factor;
            shorten = replace_balance_factor == 0;
            del_node.parent.clone().map(|parent| {
                replace_parent_key = parent;
                replace_parent_direction = del_node.direction_to_parent().unwrap();
            });
        } else {
            // Simply switch the pointer in replace parent with the child of replace.
            let direction = replace.direction_to_parent().expect("Should have parent");
            let replace_parent = self
                .get_mut_node(&replace_parent_key)
                .expect("Replace parent should exist");
            replace_parent.replace_child(&replace.key, replace_child.clone());
            replace_parent.set_child(direction.opposite(), replace_child);
            // replace should max have one child so we have to rewire the leftover child:
            self.get_mut_node(&replace.key)
                .expect("Replace should exist")
                .balance_factor = del_node.balance_factor;
            shorten = true;
        }
        ((replace_parent_key, replace_parent_direction), shorten)
    }

    /// Rewires the children of the node being deleted (`del_node`) to the replacement node (`replace`).
    ///
    /// After a node has been chosen for deletion, and another node (`replace`) has been selected to take its place,
    /// this function ensures that the children of the `del_node` are correctly reconnected to the `replace` node.
    /// This ensures that the tree maintains its structure and integrity after a node deletion.
    ///
    /// # Arguments
    /// * `del_node`: The node that is being deleted from the tree.
    /// * `replace`: The key of the node that is chosen as the replacement during deletion.
    fn rewire_delete_node_child(&mut self, del_node: &Node<K, ()>, replace: &K) {
        let children: Vec<(K, Direction)> = [Direction::Left, Direction::Right]
            .into_iter()
            .map(|d| del_node.get_child(d).zip(Some(d)))
            .filter(|k| k.is_some())
            .map(|k| k.unwrap())
            .filter(|(k, _)| k != replace)
            .collect();
        if children.len() == 0 {
            return;
        }
        children.iter().for_each(|(child, _)| {
            self.get_mut_node(child)
                .expect("Child of delete not in store but in tree")
                .parent = Some(replace.clone());
        });
        {
            let replace_node = self.get_mut_node(replace).expect("Replace should exist");
            children.into_iter().for_each(|(child, direction)| {
                replace_node.set_child(direction, Some(child));
            });
        }
    }

    /// Balances the subtree rooted at `root` by performing AVL rotations.
    ///
    /// This function determines which type of AVL balance is needed based on the balance
    /// factors of the `root` and its child in the `balance_direction`. Depending on the conditions, it then
    /// delegates to one of the three helper methods to perform the actual balancing.
    ///
    /// # Arguments
    /// * `root`: The key of the node that acts as the root of the subtree that may need balancing.
    /// * `balance_direction`: The direction (left or right) which indicates the heavier side that triggered the imbalance.
    ///
    /// # Returns
    /// Returns the new balance factor of the node after the rotations.
    fn balance(&mut self, root: &K, balance_direction: Direction) -> i32 {
        let child_id = self
            .get_node(root)
            .expect("Node should exist")
            .get_child(balance_direction)
            .expect("Child should exist");
        let child_balance_factor = self
            .get_node(&child_id)
            .expect("Child should exist")
            .balance_factor;
        if child_balance_factor.signum() == balance_direction.direction_factor() {
            self.balance_with_subtree_in_same_direction(root, &child_id, balance_direction)
        } else if child_balance_factor == 0 {
            self.balance_with_zero_bf_subtree(root, &child_id, balance_direction)
        } else {
            self.balance_with_subtree_in_different_direction(root, &child_id, balance_direction)
        }
    }

    /// Performs a single AVL rotation in the scenario where both the `root` and its `child` are imbalanced in the same direction.
    ///
    /// This function is used when the subtree that causes the imbalance (referenced by `child`)
    /// is leaning in the same direction (`imbalance_direction`) as the imbalance at the `root`. (See the example below)
    /// # Returns
    /// Returns the new balance factor of the node after the rotation.
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
        self.get_mut_node(child)
            .expect("Child in balance should exist")
            .balance_factor = 0;
        self.get_mut_node(root)
            .expect("Balance root should exist")
            .balance_factor = 0;
        self.rotate(imbalance_direction.opposite(), root, child);
        // Balance_factor of new root=child=0
        0
    }

    /// Performs a single AVL rotation when the balance factor of the child causing imbalance is zero.
    ///
    /// This scenario arises when the subtree rooted at `child` has equal depths on both sides but an
    /// imbalance at the `root` node. (See the example below, node right below C has a higher depth than the A node.)
    ///
    /// # Returns
    /// Returns the new balance factor of the node after the rotation.
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
        self.get_mut_node(root)
            .expect("Root in balance should exist")
            .balance_factor = imbalance_direction.direction_factor();
        self.get_mut_node(child)
            .expect("Child in balance should exist")
            .balance_factor = imbalance_direction.opposite().direction_factor();
        self.rotate(imbalance_direction.opposite(), root, child);
        // Balance_factor of new root=child
        imbalance_direction.opposite().direction_factor()
    }

    /// Performs a double AVL rotation to correct imbalances caused by a grandchild.
    ///
    /// This function is used when the subtree causing the imbalance is leaning in the opposite
    /// direction (`imbalance_direction`) as the imbalance at the `root` node. (See example)
    ///
    /// # Returns
    /// Returns the new balance factor of the node after the rotations.
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
        let new_root = self
            .get_node(child)
            .expect("Child of balance should exist")
            .get_child(imbalance_direction.opposite())
            .unwrap();
        let new_root_balance_factor = {
            let new_root_node = self
                .get_mut_node(&new_root)
                .expect("New root should also exist else we would not be in this case!");
            mem::replace(&mut new_root_node.balance_factor, 0)
        };

        self.change_bf_based_on_imbalance_direction(
            root,
            imbalance_direction,
            new_root_balance_factor,
        );
        self.change_bf_based_on_imbalance_direction(
            child,
            imbalance_direction.opposite(),
            new_root_balance_factor,
        );
        self.rotate(imbalance_direction, child, &new_root);
        self.rotate(imbalance_direction.opposite(), root, &new_root);
        0
    }

    /// Updates the balance factor of a node based on the balance factor of the new root and the direction of imbalance.
    ///
    /// This function adjusts the balance factor of the given `node_id` after a double rotation. The adjustment is based
    /// on the balance factor of the new root node (`new_root_balance_factor`) after the rotation and the direction
    /// (`direction`) of the original imbalance.
    fn change_bf_based_on_imbalance_direction(
        &mut self,
        node_id: &K,
        direction: Direction,
        new_root_balance_factor: i32,
    ) {
        let root = self
            .get_mut_node(node_id)
            .expect("Root in balance should exist");
        root.balance_factor = match new_root_balance_factor == direction.direction_factor() {
            false => 0,
            true => direction.opposite().direction_factor(),
        };
    }

    /// Performs a tree rotation
    ///
    /// This function rotates the subtree rooted at the node `root` in the direction `rotate_direction`,
    /// with `child` being the child node that will become the new root of the rotated subtree.
    /// The left child of root is exchanged with the right child of child or vice versa.
    /// With this one node moves into the left subtree from the right subtree or vice versa.
    /// Thus the balance factor of the subtree reduces by one or increases by one.
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
            let child = self
                .get_mut_node(child)
                .expect("Rotate without child at right position");
            child.parent = parent_key;
            left_over_child = child.get_child(rotate_direction);
            child.set_child(rotate_direction, Some(root.clone()));
        }
        if let Some(old_root_child_key) = left_over_child.as_ref() {
            self.get_mut_node(old_root_child_key)
                .expect("Child of child not in store")
                .parent = Some(root.clone());
        }
        let root = self
            .get_mut_node(root)
            .expect("Rotate without root in Store");
        root.set_child(rotate_direction.opposite(), left_over_child);
        root.parent = Some(child.clone());
    }

    fn rotate_rewire_parent(&mut self, root: &K, child: &K) -> Option<K> {
        let parent = self
            .get_node(root)
            .expect("rewire Node should exist")
            .parent
            .clone();
        parent.as_ref().map(|parent| {
            self.get_mut_node(parent)
                .expect("Parent of rotate root not in store")
                .replace_child(root, Some(child.clone()))
        });
        parent
    }
}

#[derive(ScryptoSbor, Clone)]
pub(crate) struct Node<K: ScryptoSbor, V: ScryptoSbor> {
    /// Unique key for this node
    pub(crate) key: K,
    pub(crate) value: V,
    /// The left and right children of this node in the tree
    pub(crate) left_child: Option<K>,
    pub(crate) right_child: Option<K>,
    /// The parent of this node in the tree
    pub(crate) parent: Option<K>,
    /// The next and previous nodes in double linked list. The double linked list is ordered by the keys.
    /// So to get a sorted list of all keys, we can iterate over these pointers.
    pub(crate) next: Option<K>,
    pub(crate) prev: Option<K>,
    /// Balance factor: height of right subtree - height of left subtree.
    /// The heights are never calculated, but the balance factor is updated
    /// based on the operations (insert, delete, balance) in the tree.
    pub(crate) balance_factor: i32,
}

impl<K: ScryptoSbor + Clone + Eq + Ord + Display + Debug, V: ScryptoSbor> Node<K, V> {
    /// Change the pointer of the child of this node in the given direction
    fn set_child(&mut self, direction: Direction, child: Option<K>) {
        match direction {
            Direction::Left => {
                self.left_child = child;
            }
            Direction::Right => {
                self.right_child = child;
            }
        }
    }

    /// Replace the child of this node based on the old child.
    fn replace_child(&mut self, old_child: &K, new_child: Option<K>) {
        if self.left_child == Some(old_child.clone()) {
            self.left_child = new_child;
        } else if self.right_child == Some(old_child.clone()) {
            self.right_child = new_child;
        } else {
            panic!("Tried to overwrite node but was not a child");
        }
    }

    /// Get the child of this node in the given direction
    fn get_child(&self, direction: Direction) -> Option<K> {
        match direction {
            Direction::Left => self.left_child.clone(),
            Direction::Right => self.right_child.clone(),
        }
    }

    /// Get the child of this node in the given direction
    fn get_child_in_key_direction(&self, other_key: &K) -> Option<Option<&K>> {
        match self.key.cmp(other_key) {
            Greater => Some(self.left_child.as_ref()),
            Equal => None,
            Less => Some(self.right_child.as_ref()),
        }
    }

    /// Checks if the node has any children.
    /// Returns `true` if the node has either a left or a right child, otherwise returns `false`.
    fn has_child(&self) -> bool {
        self.left_child.is_some() || self.right_child.is_some()
    }

    /// Determines the direction of the imbalance based on the balance factor.
    /// Returns the direction of the heavier subtree or `None` if the tree is balanced.
    fn get_imbalance_direction(&self) -> Option<Direction> {
        Direction::from_balance_factor(self.balance_factor)
    }

    /// Sets the node's previous or next pointer based on the provided direction.
    /// - `direction`: The direction to set (either `Left` for previous or `Right` for next).
    /// - `node`: The key of the node to be set as previous or next.
    fn set_prev_next(&mut self, direction: Direction, node: Option<K>) {
        match direction {
            Direction::Left => {
                self.prev = node;
            }
            Direction::Right => {
                self.next = node;
            }
        }
    }

    /// Retrieves the node's previous or next key based on the provided direction.
    /// Returns the key of the neighboring node in the given direction or `None` if there's no such neighbor.
    fn get_prev_next(&self, direction: Direction) -> Option<K> {
        match direction {
            Direction::Left => self.prev.clone(),
            Direction::Right => self.next.clone(),
        }
    }

    /// Determines the node's direction relative to its parent.
    /// Returns `Some(Direction)` indicating whether this node is to the left or right of its parent, or `None` if there's no parent.
    fn direction_to_parent(&self) -> Option<Direction> {
        self.parent.as_ref().map(|parent| {
            Direction::from_ordering(parent.cmp(&self.key)).expect("Nodes should be unequal")
        })
    }

    /// Determines the direction of another node relative to this node.
    /// Returns `Some(Direction)` indicating whether the other node is to the left or right of this node.
    fn direction_from_other(&self, other: K) -> Option<Direction> {
        Some(Direction::from_ordering(other.cmp(&self.key)).expect("Nodes should be unequal"))
    }

    /// Retrieves the next node's key in the linked list in the specified direction.
    /// Returns the key of the node in the given direction or `None` if there's no such node.
    fn next(&self, direction: Direction) -> Option<K> {
        match direction {
            Direction::Left => self.prev.clone(),
            Direction::Right => self.next.clone(),
        }
    }
}

pub struct ItemRef<'a, K: ScryptoSbor, V: ScryptoSbor> {
    item: KeyValueEntryRef<'a, Node<K, V>>,
}

impl<'a, K: ScryptoSbor, V: ScryptoSbor> Deref for ItemRef<'a, K, V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.item.value
    }
}

impl<K: ScryptoSbor, V: ScryptoSbor> ItemRef<'_, K, V> {
    pub fn has_next(&self) -> bool {
        self.item.next.is_some()
    }
    pub fn has_pref(&self) -> bool {
        self.item.prev.is_some()
    }
}

pub struct ItemRefMut<'a, K: ScryptoSbor, V: ScryptoSbor> {
    item: KeyValueEntryRefMut<'a, Node<K, V>>,
}

impl<K: ScryptoSbor, V: ScryptoSbor> ItemRefMut<'_, K, V> {
    pub fn has_next(&self) -> bool {
        self.item.next.is_some()
    }
    pub fn has_pref(&self) -> bool {
        self.item.prev.is_some()
    }
}

impl<K: ScryptoSbor, V: ScryptoSbor + Clone> ItemRefMut<'_, K, V> {
    pub fn get_value(&self) -> V {
        self.item.value.clone()
    }
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

/// Represents a direction, either `Left` or `Right`.
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

    /// Returns a numeric representation for the direction.
    /// These are aligned with the balance factor: positive for `Right` and negative for `Left`.
    fn direction_factor(&self) -> i32 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
        }
    }

    /// Determines the direction based on a given balance factor.
    fn from_balance_factor(balance_factor: i32) -> Option<Self> {
        match balance_factor.signum() {
            -1 => Some(Self::Left),
            1 => Some(Self::Right),
            _ => None,
        }
    }

    /// Determines the direction based on a ordering comparison
    fn from_ordering(ordering: Ordering) -> Option<Self> {
        match ordering {
            Less => Some(Self::Left),
            Greater => Some(Self::Right),
            Equal => None,
        }
    }
}

/// `NodeIterator` iterates over nodes in a doubly-linked list structure,
/// represented by the `next` and `prev` pointers, which are stored in
/// a key-value store. The nodes are traversed in a specific direction
/// until a specified bound is reached.
///
/// The iterator relies on the provided key-value store (`store`) to fetch
/// nodes by their keys. Each iteration fetches the node's value, advancing
/// the iterator based on the direction until the boundary is reached.
///
/// # Parameters
/// - `current`: The key of the current node to begin iterating from.
/// - `direction`: The direction to move in the linked list (`Left` or `Right`).
/// - `end`: The boundary key to stop iteration.
/// - `store`: The reference to the key-value store containing the linked nodes.
pub struct NodeIterator<'a, K: ScryptoSbor, V: ScryptoSbor> {
    current: Option<K>,
    direction: Direction,
    end: Bound<K>,
    store: &'a KeyValueStore<K, Node<K, V>>,
}

impl<K: ScryptoSbor + Clone, V: ScryptoSbor> NodeIterator<'_, K, V> {
    pub fn has_next(&self) -> bool {
        self.current.is_some()
    }
    pub fn next_key(&self) -> Option<K> {
        self.current.clone()
    }
}

impl<'a, K: ScryptoSbor + Clone + Ord + Eq + Display + Debug, V: ScryptoSbor + Clone> Iterator
    for NodeIterator<'a, K, V>
{
    type Item = (K, V, Option<K>);

    /// Advances the iterator to the next node and returns the value.
    ///
    /// The iterator moves in the specified direction. If the next node in that direction
    /// exists and is within the specified boundary (`end`), it fetches the value from
    /// that node. If no such node exists, or it is outside the boundary, the iterator
    /// stops and returns `None` on subsequent calls.
    fn next(&mut self) -> Option<Self::Item> {
        let current_key = self.current.clone()?;
        let node = self.store.get(&current_key).expect("Node not found");
        let next_key = node.next(self.direction);
        self.current = match next_key
            .as_ref()
            .map(|k| self.end.as_ref().within_bound(k, self.direction))
        {
            Some(true) => next_key,
            _ => None,
        };
        Some((current_key, node.value.clone(), self.current.clone()))
    }
}
pub enum IterMutControl {
    Continue,
    Break,
}

/// Mutable node iterator that implements for each
pub struct NodeIteratorMut<'a, K: ScryptoSbor, V: ScryptoSbor> {
    current: Option<K>,
    direction: Direction,
    end: Bound<K>,
    store: &'a mut KeyValueStore<K, Node<K, V>>,
}

impl<'a, K: ScryptoSbor + Clone + Ord + Eq + Display + Debug, V: ScryptoSbor + Clone>
    NodeIteratorMut<'a, K, V>
{
    /// Calls the provided function on each value in the iterator.
    ///
    /// The iterator moves in the specified direction. If the next node in that direction
    /// exists and is within the specified boundary (`end`), it fetches the value from
    /// that node. If no such node exists, or it is outside the boundary, the iterator
    /// stops and returns `None` on subsequent calls.
    ///
    /// # Parameters
    /// - `function`: The function to call on each value.
    pub fn for_each(
        &mut self,
        mut function: impl FnMut((&K, &mut V, Option<K>)) -> IterMutControl,
    ) {
        while let Some(key) = self.current.clone() {
            let mut node = self.store.get_mut(&key).expect("Node not found");
            let next = node.next(self.direction);
            self.current = match next
                .as_ref()
                .map(|k| self.end.as_ref().within_bound(k, self.direction))
            {
                Some(true) => next,
                _ => None,
            };
            match function((&key, &mut node.value, self.current.clone())) {
                IterMutControl::Continue => (),
                IterMutControl::Break => break,
            }
        }
    }
}

trait WithinBound<K> {
    fn within_bound(&self, key: &K, direction: Direction) -> bool;
}

impl<K: Ord> WithinBound<K> for Bound<&K> {
    /// Determines if the key lies within the specified boundary.
    ///
    /// - `key`: The key to check against.
    /// - `direction`: Defines if it is a left or right boundary.
    ///                So for example if the bound: Exclude(3) and the direction is right,
    ///                then the key 3 is not within the bound, and key 2 is within the bound.
    fn within_bound(&self, key: &K, direction_to_bound_from_within_range: Direction) -> bool {
        match direction_to_bound_from_within_range {
            Direction::Left => match self {
                Bound::Unbounded => true,
                Bound::Included(bound) => key >= bound,
                Bound::Excluded(bound) => key > bound,
            },
            Direction::Right => match self {
                Bound::Unbounded => true,
                Bound::Included(bound) => key <= bound,
                Bound::Excluded(bound) => key < bound,
            },
        }
    }
}
