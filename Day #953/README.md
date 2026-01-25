# Day 953

## Difficulty

Hard

## Problem Statement

Given a list of integers, write a function that returns the largest sum of non-adjacent numbers. Numbers can be `0` or negative.

Follow-up: Can you do this in O(N) time and constant space?

## Example

### Input
```
[2, 4, 6, 2, 5]
[5, 1, 1, 5]
```
### Output
```
13
10
```

## Explanation

Find the maximum sum obtainable by selecting numbers from the list such that no two chosen numbers are adjacent; for [2, 4, 6, 2, 5] pick 2, 6, 5 (13) and for [5, 1, 1, 5] pick 5 and 5 (10).

## Company

Airbnb
