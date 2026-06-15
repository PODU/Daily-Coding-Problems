# Day 1665

## Difficulty

Medium

## Problem Statement

A wall consists of several rows of bricks of various integer lengths and uniform height. Your goal is to find a vertical line going from the top to the bottom of the wall that cuts through the fewest number of bricks. If the line goes through the edge between two bricks, this does not count as a cut.

Given an input consisting of brick lengths for each row such as the one above, return the fewest number of bricks that must be cut to create a vertical line.

## Example

### Input
```
[[3, 5, 1, 1],
 [2, 3, 3, 2],
 [5, 5],
 [4, 4, 2],
 [1, 3, 3, 3],
 [1, 1, 6, 1, 1]]
```
### Output
```
2
```

## Explanation

Given rows of bricks, find the vertical cut position crossing the fewest bricks; edges between bricks don't count as cuts. The best line here (after the eighth unit) cuts only 2 bricks.

## Company

LinkedIn
