# Day 511

## Difficulty

Medium

## Problem Statement

You are given an array of integers, where each element represents the maximum number of steps that can be jumped going forward from that element. Write a function to return the minimum number of jumps you must take in order to get from the start to the end of the array.

For example, given `[6, 2, 4, 0, 5, 1, 1, 4, 2, 9]`, you should return `2`, as the optimal solution involves jumping from `6` to `5`, and then from `5` to `9`.

## Example

### Input
```
[6, 2, 4, 0, 5, 1, 1, 4, 2, 9]
```
### Output
```
2
```

## Explanation

Return the minimum number of jumps to reach the end of the array, where each value is the max jump length, using a greedy level-by-level reachability scan.

## Company

Yelp
