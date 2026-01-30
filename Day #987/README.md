# Day 987

## Difficulty

Medium

## Problem Statement

Given an array of numbers and an index `i`, return the index of the nearest larger number of the number at index `i`, where distance is measured in array indices.

If two distances to larger numbers are the equal, then return any one of them. If the array at `i` doesn't have a nearest larger integer, then return null.

Follow-up: If you can preprocess the array, can you do this in constant time?

## Example

### Input
```
[4, 1, 3, 5, 6] and index 0
```
### Output
```
3
```

## Explanation

For a given index, find the index of the nearest element (by array distance) whose value is larger, returning null if none exists.

## Company

Google
