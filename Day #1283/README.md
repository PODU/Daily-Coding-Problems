# Day 1283

## Difficulty

Medium

## Problem Statement

Given an array of integers, return a new array where each element in the new array is the number of smaller elements to the right of that element in the original input array.

For example, given the array [3, 4, 9, 6, 1], return [1, 1, 2, 1, 0], since:

 * There is 1 smaller element to the right of 3
 * There is 1 smaller element to the right of 4
 * There are 2 smaller elements to the right of 9
 * There is 1 smaller element to the right of 6
 * There are no smaller elements to the right of 1

## Example

### Input
```
[3, 4, 9, 6, 1]
```
### Output
```
[1, 1, 2, 1, 0]
```

## Explanation

For each element of the array, count how many elements to its right are smaller, returning these counts as a new array.

## Company

Google
