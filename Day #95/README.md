# Day 95

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

Find the next lexicographically greater permutation of the digits, wrapping to the smallest if none exists, ideally in place.

## Company

Palantir
