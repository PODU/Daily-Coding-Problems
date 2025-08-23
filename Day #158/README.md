# Day 158

## Difficulty

Medium

## Problem Statement

You are given an N by M matrix of 0s and 1s. Starting from the top left corner, how many ways are there to reach the bottom right corner?

You can only move right and down. 0 represents an empty space while 1 represents a wall you cannot walk through.

The top left corner and bottom right corner will always be 0.

## Example

### Input
```
[[0, 0, 1],
 [0, 0, 1],
 [1, 0, 0]]
```
### Output
```
2
```

## Explanation

Use dynamic programming counting paths into each cell from the top and left, treating walls (1) as zero ways.

## Company

Slack
