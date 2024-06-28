# scrypto-avltree

# Why
Scrypto currently does not provide a scalable `BtreeMap` since the current implementation loads the full `BtreeMap` stored in component state into memory. Meaning the amount of items you can store in the Scrypto `BtreeMap` is fairly limited, because if the `Btreemap` grows past a certain threshold the component can not be loaded anymore to execute a transaction.  That also opens attack vectors because people can flood your component trying to lock it by letting the `BtreeMap` state grow too large.

# Features
To solve that issue we have implemented an `AVL tree` as a Scrypto library, a balanced binary search tree, based on the scalable `KeyValueStore`.
Other options were using a `red black tree`. However, compared to that the `AVL tree` is optimised for lookup/query performance instead of insert/write performance which made the `AVL tree` the more appropriate fit for being used in Ociswap.

To further optimise lookups we have combined our `AVL tree` implemention with a linked list - allowing us to traverse the next
left and right nodes directly in `O(1)` without needing to traverse the tree up and down which would be `O(log n)`.

# Usage

## Example
Checkout the example folder, that provides some basic usage examples.

### Dependencies
Add avl tree to your toml config:
```toml
[dependencies]
scrypto_avltree = { git = "https://github.com/ociswap/scrypto-avltree", tag = "v1.2.0" }
```

### Instantiation 
Instantiation is rather simple:
```rust
use scrypto::prelude::*;
use scrypto_avltree::AvlTree;
let mut tree: AvlTree<Decimal, String> = AvlTree::new();
```

### Insert and get
Inserting a new key value pair is also straight forward:
```rust
tree.insert(dec!(1), "value");
```
If the key is already present in the tree, the value will be overwritten and the old value will be returned.
```rust
let old_value = tree.insert(dec!(1), "new value");
assert_eq!(old_value, Some("value"));
```

### Get and get_mut
The tree can be queried by key:
```rust
let value = tree.get(&dec!(1));
```
Or to get a mutable reference to the value:
```rust
let value = tree.get_mut(&dec!(1));
```

### Range
To iterate over the tree you can use the `range`, `range_back` methods.
It accepts a range of keys and returns an iterator over the key value pairs:
The iterator implements the standard rust iterator: it provides operations like map, for_each, fold, etc.
The range is default in rust and can have inclusive or exclusive bounds.
```rust
for (key, value, next_key) in tree.range(dec!(1)..dec!(10)) {
    info!("key: {}, value: {}", key, value);
}
```
gives you all values for the keys between 1 and 10 ascending and excluding 10.
```rust
for (key, value, next_key) in tree.range_back(Excluded(dec!(1)), Included(dec!(10))) {
    info!("key: {}, value: {}", key, value);
}
```
gives you all values for the keys between 1 and 10 descending and excluding 1.

### Mutable Range
To iterate over the tree and mutate the values you can use the `range_mut`, `range_back_mut` methods.
It accepts a range of keys and returns an iterator that can be used with the for_each callback
```rust
tree.range_mut(dec!(1)..dec!(10)).for_each(|(key, value, next_key): (&K, &mut V, Option<K>)| {
    *value=String::from("mutated");
}
for (key, value) in tree.range(dec!(1)..dec!(10)) {
    info!("key: {}, value: {}", key, value);
}
```
gives 10 times "mutated" as output.
Analogue to the `range` method the `range_back_mut` method gives you a descending iterator.

### Remove
To remove a key value pair from the tree you can use the `remove` method:
```rust
let value = tree.remove(&dec!(1));
info!("{}", value);
```
The method returns the value that was removed from the tree. 
None is returned, if the key is not present in the tree.

# Contribute
The AVL tree itself is implemented in `avl_tree.rs`. The other modules and files contain helpers for testing.
```rustup target add wasm32-unknown-unknown```
