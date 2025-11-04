# Day 549

## Difficulty

Hard

## Problem Statement

Given an array of integers where every integer occurs three times except for one integer, which only occurs once, find and return the non-duplicated integer.

Do this in O(N) time and O(1) space.

## Example

### Input
```
[6, 1, 3, 3, 3, 6, 6]
[13, 19, 13, 13]
```
### Output
```
1
19
```

## Explanation

Find the single element that appears once when all others appear three times, using O(N) time and O(1) space via bitwise counting.

## Company

Google
