# Day 748

## Difficulty

Easy

## Problem Statement

Given the root of a binary tree, find the most frequent subtree sum. The subtree sum of a node is the sum of all values under a node, including the node itself.

For example, given the following tree:

```
  5
 / \
2  -5
```

Return `2` as it occurs twice: once as the left leaf, and once as the sum of `2 + 5 - 5`.

## Example

### Input
```
  5
 / \
2  -5
```
### Output
```
2
```

## Explanation

For each node, compute its subtree sum (sum of all values in its subtree); return the subtree sum value that occurs most frequently.

## Company

Apple
