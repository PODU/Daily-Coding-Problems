# Day 789

## Difficulty

Hard

## Problem Statement

Given a number represented by a list of digits, find the next greater permutation of a number, in terms of lexicographic ordering. If there is not greater permutation possible, return the permutation with the lowest value/ordering.

Can you perform the operation without allocating extra memory (disregarding the input memory)?

## Example

### Input
```
[1,2,3]
[1,3,2]
[3,2,1]
```
### Output
```
[1,3,2]
[2,1,3]
[1,2,3]
```

## Explanation

Compute the next lexicographic permutation of the digit list in place, wrapping to the smallest permutation when none is greater.

## Company

Palantir
