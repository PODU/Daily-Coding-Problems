# Day 513

## Difficulty

Medium

## Problem Statement

Given a list of integers and a number K, return which contiguous elements of the list sum to K.

For example, if the list is [1, 2, 3, 4, 5] and K is 9, then it should return [2, 3, 4], since 2 + 3 + 4 = 9.

## Example

### Input
```
[1, 2, 3, 4, 5], K = 9
```
### Output
```
[2, 3, 4]
```

## Explanation

Find a contiguous subarray that sums to K, typically using a running prefix-sum with a sliding window or hash map.

## Company

Lyft
