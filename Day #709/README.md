# Day 709

## Difficulty

Medium

## Problem Statement

You are given an N by M matrix of `0`s and `1`s. Starting from the top left corner, how many ways are there to reach the bottom right corner?

You can only move right and down. `0` represents an empty space while `1` represents a wall you cannot walk through.

The top left corner and bottom right corner will always be `0`.

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

Count the number of distinct paths from the top-left to the bottom-right of a grid, moving only right or down and avoiding cells marked as walls.

## Company

Slack
