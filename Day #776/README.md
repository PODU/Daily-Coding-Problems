# Day 776

## Difficulty

Easy

## Problem Statement

There are `N` prisoners standing in a circle, waiting to be executed. The executions are carried out starting with the `k`th person, and removing every successive `k`th person going clockwise until there is no one left.

Given `N` and `k`, write an algorithm to determine where a prisoner should stand in order to be the last survivor.

Bonus: Find an `O(log N)` solution if `k = 2`.

## Example

### Input
```
N = 5, k = 2
```
### Output
```
3
```

## Explanation

The Josephus problem: with N=5 and k=2 the execution order is [2, 4, 1, 5, 3], so position 3 survives.

## Company

Bloomberg
