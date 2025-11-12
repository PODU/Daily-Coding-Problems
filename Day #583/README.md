# Day 583

## Difficulty

Medium

## Problem Statement

You are given a 2-d matrix where each cell represents number of coins in that cell. Assuming we start at matrix[0][0], and can only move right or down, find the maximum number of coins you can collect by the bottom right corner.

## Example

### Input
```
0 3 1 1
2 0 0 4
1 5 3 1
```
### Output
```
The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = 12 coins.
```

## Explanation

Use dynamic programming to find the path from the top-left to the bottom-right corner, moving only right or down, that collects the maximum number of coins.

## Company

Zillow
