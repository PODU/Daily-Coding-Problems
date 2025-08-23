# Day 156

## Difficulty

Medium

## Problem Statement

Given a positive integer n, find the smallest number of squared integers which sum to n.

## Example

### Input
```
n = 13
n = 27
```
### Output
```
2 (since 13 = 3^2 + 2^2 = 9 + 4)
3 (since 27 = 3^2 + 3^2 + 3^2 = 9 + 9 + 9)
```

## Explanation

Use dynamic programming where dp[i] is the minimum count of perfect squares summing to i.

## Company

Facebook
