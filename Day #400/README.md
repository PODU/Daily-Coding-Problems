# Day 400

## Difficulty

Hard

## Problem Statement

Given a list of numbers `L`, implement a method `sum(i, j)` which returns the sum from the sublist `L[i:j]` (including `i`, excluding `j`).

You can assume that you can do some pre-processing. `sum()` should be optimized over the pre-processing step.

## Example

### Input
```
L = [1, 2, 3, 4, 5], sum(1, 3)
```
### Output
```
5
```

## Explanation

Implement a range-sum query sum(i, j) over a list, optimizing the query time using pre-processing; sum(1, 3) returns sum([2, 3]) = 5.

## Company

Goldman Sachs
