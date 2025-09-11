# Day 254

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

Transform a binary tree into a full binary tree by deleting every node that has exactly one child, reconnecting its subtree appropriately.

## Company

Yahoo
