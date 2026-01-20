# Day 926

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

Compute every node's subtree sum and return the most frequently occurring sum value.

## Company

Apple
