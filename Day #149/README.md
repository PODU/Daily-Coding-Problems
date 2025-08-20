# Day 149

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

Build a structure (e.g. prefix sums) so that range-sum queries over a list can be answered efficiently after preprocessing.

## Company

Goldman Sachs
