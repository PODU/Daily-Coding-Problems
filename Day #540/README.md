# Day 540

## Difficulty

Easy

## Problem Statement

In Ancient Greece, it was common to write text with the first line going left to right, the second line going right to left, and continuing to go back and forth. This style was called "boustrophedon".

Given a binary tree, write an algorithm to print the nodes in boustrophedon order.

## Example

### Input
```
       1
    /     \
  2         3
 / \       / \
4   5     6   7
```
### Output
```
[1, 3, 2, 4, 5, 6, 7]
```

## Explanation

Traverse a binary tree level by level, alternating direction each level (zigzag / boustrophedon order).

## Company

Morgan Stanley
