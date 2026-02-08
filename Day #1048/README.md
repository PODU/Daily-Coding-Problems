# Day 1048

## Difficulty

Hard

## Problem Statement

A Cartesian tree with sequence `S` is a binary tree defined by the following two properties:

* It is heap-ordered, so that each parent value is strictly less than that of its children.
* An in-order traversal of the tree produces nodes with values that correspond exactly to `S`.

Given a sequence `S`, construct the corresponding Cartesian tree.

## Example

### Input
```
[3, 2, 6, 1, 9]
```
### Output
```
      1
    /   \
  2       9
 / \
3   6
```

## Explanation

Build a Cartesian tree from a sequence: a binary tree that is min-heap-ordered while its in-order traversal reproduces the original sequence.

## Company

Netflix
