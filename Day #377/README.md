# Day 377

## Difficulty

Hard

## Problem Statement

Given an array of numbers `arr` and a window of size `k`, print out the median of each window of size `k` starting from the left and moving right by one position each time.

Recall that the median of an even-sized list is the average of the two middle numbers.

## Example

### Input
```
[-1, 5, 13, 8, 2, 3, 3, 1], k = 3
```
### Output
```
5 <- median of [-1, 5, 13]
8 <- median of [5, 13, 8]
8 <- median of [13, 8, 2]
3 <- median of [8, 2, 3]
3 <- median of [2, 3, 3]
3 <- median of [3, 3, 1]
```

## Explanation

Compute the sliding-window median of an array for every contiguous window of size k as the window moves one step at a time.

## Company

Microsoft
