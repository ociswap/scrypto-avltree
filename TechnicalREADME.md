# Technical insights to the AVL Tree
### Introduction
The AVL tree, named after its inventors Adelson-Velsky and Landis, is a self-balancing binary search tree. 
While similar in concept to a Red-Black tree, the AVL tree prioritizes lookup operations, ensuring that they remain efficient by keeping the tree height balanced. 
This contrasts with the Red-Black tree, which, while also balanced, might allow slightly more depth, trading off a bit of lookup efficiency for faster insert/write operations. 
In essence, an AVL tree is optimized for lookup performance, balancing itself automatically after each insert or delete operation.

### AVL Property
A key feature distinguishing the AVL tree is its balance factor. 
This factor is calculated as the difference in heights between a node's left and right subtrees. 
An AVL tree maintains the invariant that this factor can only be -1, 0, or 1 for every node.
This ensures the tree remains balanced, preventing skewed formations that would hinder performance by a high depth of the tree.

## Tree structure
The tree is constructed atop Scrypto's KVStore, which facilitates the lazy loading of nodes. 
This design choice means nodes are loaded into the cache exclusively when necessary.

In this architecture, a node doesn't house the actual subtree. 
Instead, it retains pointers to the left and right subtrees. 
Moreover, for enhanced sorted iteration capabilities, each node is equipped with pointers to both its subsequent and preceding nodes in the tree. 
This doubly linked configuration permits sorted iteration in O(k) time, where 'k' represents the number of nodes to traverse, a significant refinement over the O(log(n) * K) approach.

In essence, each node comprises pointers to its immediate left and right children, the adjacent nodes in the double linked list, and its parent node. 
The tree's inherent balance is maintained using a balance factor. 
This factor is the computed difference in heights between a node's left and right subtrees.

To further optimize the tree's performance, an in-built caching mechanism is deployed. 
This cache, implemented as a HashMap, preserves each node alongside its pointers and respective node key.
Importantly, to reflect structural changes, this cache is purged after every operation.

## Operations
There are two types of operations, altering and non-altering operations.
Altering operations change the structure of the tree and these changes are complex, thus they are described in detail.

### Insert
The insert operation is done in two steps: 
1. **Node Placement:** Identify the appropriate position within the tree for the new node and insert it there.
2. **Balance Mainteance:** Post insertion, it's crucial to update the balance factors for all nodes from the newly inserted node's position up to the root. If the absolute value of a balance factor exceeds 1, the tree requires rebalancing.

In the code, after the first step, a 'parent' node is identified, serving as the starting point for the rebalancing procedure. 
As for the balance factor updates, the algorithm iterates through each parent node until one of two conditions are met:

 - The tree's end is reached.
 - A node is encountered which is unaffected by the insertion.

The latter scenario arises when a parent node's balance factor is either 1 or -1. In such cases, the tree either:

 - Attains balance, with no changes to its overall depth.
 - Moves from a slight imbalance of 1 unit to a perfectly balanced state (without depth alteration).

Illustrative Example:
node `a` and `b` are already in the tree and node `i` is the node that is inserted
````
    a|1
   / \
      b|0
       \
       i|0
After updated and rebalance
    b|0
   / \
  a|0 i|0
  ````
In this scenario, node `i` is introduced. The rebalancing process halts at node `a`, given its balance factor was 1, resulting in the tree becoming balanced.

Lastly, when updating the double linked list during the insert operation, the process is straightforward: the node's parent becomes one of its neighbors, while its other neighbor is the former neighbor of the parent.### Delete
### Delete
The process of deleting a node from the AVL tree involves three primary steps:
1. **Node Replacement:** If the node to delete has children, replace it with its successor or predecessor, ensuring the structure of the tree. Following this, update all pointers of the replaced node.
2. **Pointer Updates:** Revise all pointers within the tree, as well as within the double linked list, to ensure that they correctly reference the replaced node. If there's no replacement node, remove all pointers directing to the deleted node.
3. **Maintaining Tree Balance:** After removal, it's essential to adjust the balance factors of nodes, starting from the parent of the deleted or replaced node and moving up to the root. If a balance factor is not -1, 0, or 1, rebalancing becomes necessary.

The first step is to find a replace node, which can be the successor or predecessor node, if they are children of the deleted node.
Additionally, the pointers to the replace node are updated, with jumping over it in the double linked list, and substituting it with a child in the tree.
A replace is either a leaf node (no children) or has one child. 
If there would be two children, the replace node could not be a neighbour of the delete node, because on of the children would be the direct neighbour of the delete node.
If the deleted node has no children, the node is just deleted, without replacement.
In the second step all the pointers to the delete node are updated, that all neighbours, children and parents point to the replace node.
If there is no replace node, the pointers to the deleted node are removed from other nodes.
After the node is removed from the tree, the balance factors are updated upwards from the replace node parent or the delete node parent.
The balance factors are updated until the root is reached or the balance factor of a parent is 0. 
If a node is zero the shortening of the subtree does not affect nodes higher up in the tree, because the subtree has the same height as before because it still has the other side of the tree.

Example:
````
    a|0
   / \
  x   b|1
 / \    \
x   x   d|0
Node to delete d
    a|-1
   / \
  x   b|0
 / \
x   x  
````
The update reduces `b` to 0 and the update continues with `a`, which is now -1.
Since `a` was 0 before, the tree the update stops at `a`.
### Conclusion

The AVL tree, with its self-balancing properties, ensures efficient lookup operations, making it an ideal choice for scenarios where read performance is paramount. The implementation provided leverages scrypto's KVStore for lazy loading and integrates caching mechanisms to further optimize operations. For details on the AVL tree's methods and their underlying mechanics, refer to the provided function docstrings and inline comments.
For further insight see (https://en.wikipedia.org/wiki/AVL_tree, https://www.geeksforgeeks.org/introduction-to-avl-tree/)