# Day 723

## Difficulty

Medium

## Problem Statement

Given a set of closed intervals, find the smallest set of numbers that covers all the intervals. If there are multiple smallest sets, return any of them.

## Example

### Input
```
[0, 3], [2, 6], [3, 4], [6, 9]
```
### Output
```
{3, 6}
```

## Explanation

Find a minimum-size set of points such that every given closed interval contains at least one chosen point, solvable greedily by sorting on interval end.

## Company

Google
