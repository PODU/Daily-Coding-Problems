# Day 1215

## Difficulty

Easy

## Problem Statement

Given a positive integer N, find the smallest number of steps it will take to reach 1.

There are two kinds of permitted steps:

 * You may decrement N to N - 1.
 * If a * b = N, you may decrement N to the larger of a and b.

## Example

### Input
```
100
```
### Output
```
5
```

## Explanation

Find the minimum number of moves to reduce N to 1, where each move either subtracts 1 or, given a factorization a * b = N, replaces N with the larger factor. For 100, one optimal route is 100 -> 10 -> 9 -> 3 -> 2 -> 1, taking five steps.

## Company

PagerDuty
