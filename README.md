# scrypto-avltree

# Why
Scrypto currently does not provide a scalable `BtreeMap` since the current implementation loads the full `BtreeMap` stored in component state into memory. Meaning the amount of items you can store in the Scrypto `BtreeMap` is fairly limited, because if the `Btreemap` grows past a certain threshold the component can not be loaded anymore to execute a transaction.  That also opens attack vectors because people can flood your component trying to lock it by letting the `BtreeMap` state grow too large.

# Features
To solve that issue we have implemented an `AVL tree` as a Scrypto library, a balanced binary search tree, based on the scalable `KeyValueStore`.
Other options were using a `red black tree`. However, compared to that the `AVL tree` is optimised for lookup/query performance instead of insert/write performance which made the `AVL tree` the more appropriate fit for being used in Ociswap.

To further optimise lookups we have combined our `AVL tree` implemention with a linked list - allowing us to traverse the next
left and right nodes directly in `O(1)` without needing to traverse the tree up and down which would be `O(log n)`.

# Contribute
The AVL tree itself is implemented in `avl_tree.rs`. The other modules and files contain helpers for testing.