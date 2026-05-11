# Day 1508

## Difficulty

Hard

## Problem Statement

Given an array of integers, return a new array such that each element at index `i` of the new array is the product of all the numbers in the original array except the one at `i`.

For example, if our input was `[1, 2, 3, 4, 5]`, the expected output would be `[120, 60, 40, 30, 24]`. If our input was `[3, 2, 1]`, the expected output would be `[2, 3, 6]`.

Follow-up: what if you can't use division?

## Example

### Input
```
[1, 2, 3, 4, 5]
```
### Output
```
[120, 60, 40, 30, 24]
```

## Explanation

Produce an array where each element is the product of all other elements, with a follow-up to do it without using division.

## Company

Uber
