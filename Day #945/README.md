# Day 945

## Difficulty

Hard

## Problem Statement

Given a tree where each edge has a weight, compute the length of the longest path in the tree.

The path does not have to pass through the root, and each node can have any amount of children.

## Example

### Input
```
   a
  /|\
 b c d
    / \
   e   f
  / \
 g   h

weights: a-b: 3, a-c: 5, a-d: 8, d-e: 2, d-f: 4, e-g: 1, e-h: 1
```
### Output
```
17 (path c -> a -> d -> f)
```

## Explanation

Given a weighted tree, find the length of the longest path between any two nodes, where the path need not pass through the root.

## Company

Uber
