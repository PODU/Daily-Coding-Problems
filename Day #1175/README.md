# Day 1175

## Difficulty

Medium

## Problem Statement

Given a circular array, compute its maximum subarray sum in O(n) time. A subarray can be empty, and in this case the sum is 0.

## Example

### Input
```
[8, -1, 3, 4]
[-4, 5, 1, 0]
```
### Output
```
15
6
```

## Explanation

Find the maximum subarray sum in a circular array (where subarrays may wrap around) in linear time. For [8, -1, 3, 4] we choose 3, 4, and 8 wrapping around; for [-4, 5, 1, 0] we choose 5 and 1.

## Company

Facebook
