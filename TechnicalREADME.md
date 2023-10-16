# Technical readme

### Introduction
The AVL tree, named after its inventors Adelson-Velsky and Landis, is a self-balancing binary search tree. 
While similar in concept to a Red-Black tree, the AVL tree prioritizes lookup operations, ensuring that they remain efficient by keeping the tree height balanced. 
This contrasts with the Red-Black tree, which, while also balanced, might allow slightly more depth, trading off a bit of lookup efficiency for faster insert/write operations. 
In essence, an AVL tree is optimized for lookup performance, balancing itself automatically after each insert or delete operation.

### AVL Property
A key feature distinguishing the AVL tree is its balance factor. 
This factor is calculated as the difference in heights between a node's left and right subtrees (children of the node). 
An AVL tree maintains the invariant that this factor can only be -1, 0, or 1 for every node.
This ensures the tree remains balanced, preventing skewed formations that would hinder performance by a high depth of the tree.

## Tree structure
The tree is constructed atop Scrypto's KVStore, which facilitates the lazy loading of nodes. 
This design choice means nodes are loaded into the cache exclusively when necessary.

In this architecture, a node doesn't store the actual subtree. 
Instead, it retains pointers to the left and right subtrees. 
Furthermore, for enhanced sorted iteration capabilities, each node is equipped with pointers to both its subsequent and preceding nodes in the tree. 
This doubly linked configuration permits sorted iteration in O(k) time, where 'k' represents the number of nodes to traverse, a significant refinement over the O(log(n) + K) approach.

In essence, each node comprises pointers to its immediate left and right child (subtrees), the adjacent nodes in the double linked list, and its parent node.
The direct predecessor and successor of a node in the double linked list will be called the neighbours of the node. 
[//]: # (The tree's inherent balance is maintained using a balance factor. )
[//]: # (This factor is the computed difference in heights between a node's left and right subtrees.)

To further optimize the tree's performance, an in-built caching mechanism is deployed. 
This cache, implemented as a HashMap, preserves each node alongside its pointers and respective node key.
Importantly, to reflect structural changes, this cache is synced with the KVStore and cleared after every operation.

## Operations
There are two types of operations, altering and non-altering operations.
Altering operations change the structure of the tree and these changes are complex, thus they are described in detail.

### Insert
The insert operation is done in two steps: 
1. **Node Placement:** Identify the appropriate position within the tree for the new node and insert it there.
2. **Balance Mainteance:** Post insertion, it's crucial to update the balance factors for all nodes from the newly inserted node's position up to the root. If the absolute value of a balance factor exceeds 1, the tree requires rebalancing.

In the code, after completing the first step, we identify a 'parent' node. This node serves as the starting point for the rebalancing procedure.

Regarding balance factor updates, the algorithm iterates through each parent node, adjusting its balance factor either by increasing it by 1 or decreasing it by 1. The logic is as follows:

- If the inserted node is in the right subtree, the balance factor increases.
- If the inserted node is in the left subtree, the balance factor decreases.
This updating process continues up the tree until one of the following conditions is met:

The end of the tree is reached.
A node is encountered that remains unaffected by the insertion.
The second condition occurs when a node's balance factor is either 1 or -1 before the insertion. Under such circumstances, the tree either:

- Transitions from a slight imbalance of 1 unit to a perfectly balanced state, without any change in depth.
- Shifts from a slight imbalance of 1 unit to a slight imbalance of 2 units. In this situation, the tree becomes balanced, and its height remains unchanged from before the insertion.

Therefore, the overall height of the tree increases due to an insertion only if all nodes—from the inserted node to the root—have a balance factor of 0. This makes intuitive sense. If there were a node with a different balance factor, the tree could be rebalanced, preventing any increase in height.

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

Lastly, when updating the double linked list during the insert operation, the process is straightforward: the node's parent becomes one of its neighbours, while its other neighbour is the former neighbour of the parent.
### Delete
The process of deleting a node from the AVL tree involves three primary steps:
1. **Node Replacement:** If the node to delete has children, replace it with its successor or predecessor, to ensure the structure of the tree. Following this, update all pointers to the replacement node.
2. **Pointer Updates:** Update all pointers within the tree, as well as within the double linked list, to ensure that they correctly reference the replacement node. If there's no replacement node, remove all pointers directing to the deleted node.
3. **Maintaining Tree Balance:** After removal, it's essential to adjust the balance factors of nodes, starting from the parent of the deleted or replacement node and moving up to the root. If a balance factor is not -1, 0, or 1, rebalancing becomes necessary.

#### Replacement node
The first step is to find a replacement node, which can be the successor or predecessor.
If the delete node has no children, it does not need to be replaced. An example with children is shown below.
````
    d
   / \
  _    _
 / \  / \
_   r r  _
````
Both nodes with r can be the replacement node.
The replacement node is determined by the balance factor of the deleted node; so, that the deleted node does not have to be balanced after the replacement.
The successor of the node is picked, if the balance factor of the deleted node is -1, and the predecessor is picked if the balance factor is 1.
Removing the replacement node from its original position creates another hole in the tree that needs to be filled.

The replacement node has either no children or only one child.
If it has no children, the hole does not need to be filled.
If the replacement node has one child, the replacement node is skipped in the tree and the parent of the replacement node points to the child of the replacement node and vice versa.
If there would be two children, the replacement node could not be a direct neighbor of the deleted node in the linked list because one of the children would be the direct neighbor of the deleted node.

Why a replacement node has max one child

Example:
````
    d
   / \
  x   ...
       / \ 
      r 	
     / \
    _   _
````
Assume `r` is the replacement node and the successor of `d`, and `r` has two children.
The left child of `r` is smaller than `r` and bigger than `d`; thus, `r` is not a successor node of `d`, but some node in the left subtree of `r` is the successor of the delete node.
Thus, `r` was not a replacement node in the first place. A similar argument can be made for the predecessor.

When the replacement node is found, the pointers to the replacement node are either set to None or updated to point to the child of the replacement node.

#### Delete Pointer Updates
1. Updating Pointers to the Replacement Node:
   - All pointers previously pointing to the deleted node are redirected to its replacement.
   - The replacement node's pointers are updated to point to the neighbors, children, and parent of the deleted node.

2. Handling No Replacement Scenario:
   - If there isn't a replacement node, all pointers to the deleted node are removed.
   - Specifically, the parent's pointer to the deleted node is nullified.
   - Neighboring nodes in the linked list bypass the deleted node, pointing directly to each other.

#### Balance factor updates
After the node is removed from the tree, the balance factors are updated upwards from the replacement node's parent or the delete node's parent.
Update of the balance factors means, that the balance factor of a node is reduced or increased by one, dependent on the direction of the subtree where the height was reduced.
If the deletion was in the left subtree, we increase the balance factor by one, and if the deletion was in the right subtree, we reduce the balance factor by one.
The balance factors are updated until the root is reached or the balance factor of a node is 0 before the update.
When the balance factor of a node is 0 before the update, the node had two subtrees with the same height.
Through the deletion of a node, one of the subtrees reduced its height by one, and the balance factor of the node is now 1 or -1.
The height of the node is not reduced because the other subtree still has the same height. 
Thus, the nodes further up in the tree are not effected by the node deletion.

The following is an example of a delete operation.

````
     /
    a|0
   / \
  x   b|1
 / \    \
x   x   d|0

Node to delete d

     /
    a|-1
   / \
  x   b|0
 / \
x   x  
````
The update reduces `b` to 0, and the update continues with `a`, which is now -1.
Since `a` was 0 before, the tree the update stops after it updated `a` and does not continue to the parents of `a`.
### Conclusion
The AVL tree with its self-balancing properties, ensures efficient lookup operations, making it an ideal choice for scenarios where read performance is paramount. The implementation provided leverages Scrypto's KVStore for lazy loading and integrates caching mechanisms to further optimize lookups. For details on the AVL tree's methods and theunderlying mechanics, refer to the provided function docstrings and inline comments.
For further insight see (https://en.wikipedia.org/wiki/AVL_tree, https://www.geeksforgeeks.org/introduction-to-avl-tree/)
