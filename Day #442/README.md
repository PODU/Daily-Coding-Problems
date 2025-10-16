# Day 442

## Difficulty

Hard

## Problem Statement

A Cartesian tree with sequence `S` is a binary tree defined by the following two properties:

- It is heap-ordered, so that each parent value is strictly less than that of its children.
- An in-order traversal of the tree produces nodes with values that correspond exactly to `S`.

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

Construct the Cartesian tree (heap-ordered, in-order traversal equals the sequence) from a given sequence.

## Company

Netflix
