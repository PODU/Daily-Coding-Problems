# Day 1235

## Difficulty

Medium

## Problem Statement

Given the root to a binary tree, implement `serialize(root)`, which serializes the tree into a string, and `deserialize(s)`, which deserializes the string back into the tree.

For example, given the following `Node` class

```
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```

The following test should pass:

```
node = Node('root', Node('left', Node('left.left')), Node('right'))
assert deserialize(serialize(node)).left.left.val == 'left.left'
```

## Example

### Input
```
node = Node('root', Node('left', Node('left.left')), Node('right'))
```
### Output
```
deserialize(serialize(node)).left.left.val == 'left.left'
```

## Explanation

Implement serialize and deserialize functions for a binary tree so that serializing then deserializing reconstructs the original tree structure and values.

## Company

Google
