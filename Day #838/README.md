# Day 838

## Difficulty

Medium

## Problem Statement

You are given a 2-d `matrix` where each cell represents number of coins in that cell. Assuming we start at `matrix[0][0]`, and can only move right or down, find the maximum number of coins you can collect by the bottom right corner.

For example, in this matrix

The most we can collect is 0 + 2 + 1 + 5 + 3 + 1 = 12 coins.

## Example

### Input
```
0 3 1 1
2 0 0 4
1 5 3 1
```
### Output
```
12
```

## Explanation

Starting at the top-left of a grid and moving only right or down, find the maximum sum of coin values collected on the way to the bottom-right.

## Company

Zillow
