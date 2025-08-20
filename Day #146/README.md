# Day 146

## Difficulty

Medium

## Problem Statement

Given a binary tree where all nodes are either `0` or `1`, prune the tree so that subtrees containing all `0`s are removed.

We do not remove the tree at the root or its left child because it still has a `1` as a descendant.

## Example

### Input
```
   0
  / \
 1   0
    / \
   1   0
  / \
 0   0
```
### Output
```
   0
  / \
 1   0
    /
   1
```

## Explanation

In a binary tree of 0s and 1s, remove every subtree whose nodes are all 0.

## Company

BufferBox
