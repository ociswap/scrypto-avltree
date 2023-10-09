# Technical insights to the AVL Tree
## Introduction
The tree is similar to a black red tree, that it is a balanced binary tree. 
The difference is that the AVL tree is more optimised for lookup performance instead of insert/write performance.
The AVL tree will automatically balance itself after each insert/write operation.

## Tree structure
As described in the general readme, the tree is implemented with the KVStore of  scrypto to enable lazy loading of the node.
A node is only loaded into cache if it is needed.
This a node only contains pointers to the subtrees and not the actual subtree.
To improve sorted iteration over the nodes, each node has a pointer to the next and previous node in the tree. 
This double linked list allows for a sorted iteration in O(k) lookups, where k is the number of nodes to iterate and the start node is in the tree not a lower bound (instead of O(log(n) * K)).
In summary, each node has a pointer to the left and right subtree, the next and previous node in the tree and the parent node.
Additionally, the tree is balanced, so the height of the left and right subtree differs by at most 1.
This is done through a balance factor, the balance factor is the height difference between the two subtrees.

To improve the performance of the tree, the tree is implemented with a cache.
The cache is a HashMap, which stores a node with its pointers and the key of the node.
This cache is emptied at the end of each operation, as the tree might have changed.

## Operations
There are two types of operations, altering and non-altering operations.
Altering operations change the structure of the tree, while non-altering operations only read from the tree.

### Insert
Is a altering operation, as it inserts a new node into the right position in the tree.
The insert operation is done in two steps: 
1. Find the right position in the tree and insert the new node.
2. Update the balance factors of the nodes on the path from the inserted node to the root. If the balance factor is not -1, 0 or 1, the tree has to be rebalanced.

In the code, the first step gives a parent that is the starting point for the re-balancing.
The updating of the balance factors takes this parents and iterates over the parents until the end of the tree is reached or the parents are not longer affected by this altering of the tree.
The second case appears when the balance factor of a parent was 1 or -1, because then either the tree gets balanced (no increase of the depth) or the tree wa unbalanced with 1 and gets equalized (depth also does not increase)
Example:
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
 Inserted node is i and the loop stopped at a because the balance factor of a was 1 and the tree got balanced.

The update of the double linked list in insert is quite trivial: one neighbour is the parent and the other neighbour in the double linked list is the old neighbour of the parent.
### Delete
The delete operation is a bit more complex than the insert operation.
The delete operation is also done in two steps:
1. Replace the node to delete with the successor/predecessor node if they are children of the deleted node, and update all pointers of the replaced node.
2. Update all pointers in the tree and the double linked list.
3. Update the balance factors of the nodes on the path from the deleted node parent or replaced node parent to the root. If the balance factor is not -1, 0 or 1, the tree has to be rebalanced.

The first step is to find a replace node, which can be the successor or predecessor node, if they are children of the deleted node.
If the deleted node has no children, the node is just deleted, without replacement.
In the second step all the pointers are updated that all neighbours, children and parents point to the replace node.
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
The update reduces 'b' to 0 and the update continues with a, which is now -1.
Since 'a' was 0 before, the tree the update stops at a.
