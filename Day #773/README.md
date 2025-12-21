# Day 773

## Difficulty

Medium

## Problem Statement

We can determine how "out of order" an array A is by counting the number of inversions it has. Two elements `A[i]` and `A[j]` form an inversion if `A[i] > A[j]` but `i < j`. That is, a smaller element appears after a larger element.

Given an array, count the number of inversions it has. Do this faster than O(N^2) time.

You may assume each element in the array is distinct.

## Example

### Input
```
[2, 4, 1, 3, 5]
[5, 4, 3, 2, 1]
```
### Output
```
3
10
```

## Explanation

A sorted list has zero inversions; [2,4,1,3,5] has three inversions (2,1), (4,1), (4,3); [5,4,3,2,1] has ten. Count inversions faster than O(N^2), typically via a modified merge sort.

## Company

Google
