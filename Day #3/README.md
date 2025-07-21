# Day 3

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

Implement functions to convert a binary tree to a string and back, preserving its structure.

## Company

Google
