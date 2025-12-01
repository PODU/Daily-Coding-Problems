# Day 679

## Difficulty

Medium

## Problem Statement

We can determine how "out of order" an array A is by counting the number of inversions it has. Two elements `A[i]` and `A[j]` form an inversion if `A[i] > A[j]` but `i < j`. That is, a smaller element appears after a larger element.

Given an array, count the number of inversions it has. Do this faster than O(N^2) time.

You may assume each element in the array is distinct.

For example, a sorted list has zero inversions. The array [2, 4, 1, 3, 5] has three inversions: (2, 1), (4, 1), and (4, 3). The array [5, 4, 3, 2, 1] has ten inversions: every distinct pair forms an inversion.

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

Count the number of inversion pairs (a larger element appearing before a smaller one) in an array, in better than O(N^2) time (e.g. via a modified merge sort).

## Company

Google
