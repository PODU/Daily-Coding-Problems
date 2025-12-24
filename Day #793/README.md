# Day 793

## Difficulty

Medium

## Problem Statement

Recall that a full binary tree is one in which each node is either a leaf node, or has two children. Given a binary tree, convert it to a full one by removing nodes with only one child.

## Example

### Input
```
         0
      /     \
    1         2
  /            \
3                 4
  \             /   \
    5          6     7
```
### Output
```
     0
  /     \
5         4
        /   \
       6     7
```

## Explanation

Prune the binary tree so every remaining node is a leaf or has two children, by removing single-child nodes and reconnecting their subtrees.

## Company

Yahoo
