# Day 672

## Difficulty

Easy

## Problem Statement

You are given an array of arrays of integers, where each array corresponds to a row in a triangle of numbers. For example, `[[1], [2, 3], [1, 5, 1]]` represents the triangle:

```
  1
 2 3
1 5 1
```

We define a path in the triangle to start at the top and go down one row at a time to an adjacent value, eventually ending with an entry on the bottom row. For example, 1 -> 3 -> 5. The weight of the path is the sum of the entries.

Write a program that returns the weight of the maximum weight path.

## Example

### Input
```
[[1], [2, 3], [1, 5, 1]]
```
### Output
```
9
```

## Explanation

Given a triangle of numbers, find the maximum sum path from the top to the bottom row, moving to an adjacent value on each step down.

## Company

Google
