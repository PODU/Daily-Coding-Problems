# Day 1274

## Difficulty

Medium

## Problem Statement

Given a binary tree where all nodes are either 0 or 1, prune the tree so that subtrees containing all 0s are removed.

For example, given the following tree:

```
   0
  / \
 1   0
    / \
   1   0
  / \
 0   0
```

should be pruned to:

```
   0
  / \
 1   0
    /
   1
```

We do not remove the tree at the root or its left child because it still has a 1 as a descendant.

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

Prune a binary tree of 0/1 nodes by removing every subtree that contains only 0s.

## Company

BufferBox
