# Day 1356

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

Perform a zigzag (boustrophedon) level-order traversal of a binary tree, alternating direction on each level.

## Company

Morgan Stanley
