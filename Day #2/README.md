# Day 2

## Difficulty

Hard

## Problem Statement

Given an array of integers, return a new array such that each element at index `i` of the new array is the product of all the numbers in the original array except the one at `i`.

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

Build a new array where each position holds the product of every other element. For [1, 2, 3, 4, 5] the output is [120, 60, 40, 30, 24].

## Company

Uber
