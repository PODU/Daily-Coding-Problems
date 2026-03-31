# Day 1284

## Difficulty

Hard

## Problem Statement

Let A be an N by M matrix in which every row and every column is sorted.

Given i1, j1, i2, and j2, compute the number of elements of M smaller than M[i1, j1] and larger than M[i2, j2].

## Example

### Input
```
[[1, 3, 7, 10, 15, 20],
 [2, 6, 9, 14, 22, 25],
 [3, 8, 10, 15, 25, 30],
 [10, 11, 12, 23, 30, 35],
 [20, 25, 30, 35, 40, 45]]
i1 = 1, j1 = 1, i2 = 3, j2 = 3
```
### Output
```
15
```

## Explanation

In a row- and column-sorted matrix, count how many elements are smaller than M[i1, j1] or larger than M[i2, j2].

## Company

Google
