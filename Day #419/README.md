# Day 419

## Difficulty

Easy

## Problem Statement

Given a positive integer N, find the smallest number of steps it will take to reach 1.

There are two kinds of permitted steps:

 * You may decrement N to N - 1.
 * If a * b = N, you may decrement N to the larger of a and b.

For example, given 100, you can reach 1 in five steps with the following route: 100 -> 10 -> 9 -> 3 -> 2 -> 1.

## Example

### Input
```
N = 100
```
### Output
```
5  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)
```

## Explanation

Find the minimum number of steps to reduce N to 1, where each step either decrements by one or replaces N with the larger factor of a factor pair.

## Company

PagerDuty
