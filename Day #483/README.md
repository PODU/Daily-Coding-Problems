# Day 483

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
3 (the order of executions would be [2, 4, 1, 5, 3])
```

## Explanation

Solve the Josephus problem: given N people in a circle and a step size k, determine the position of the last survivor.

## Company

Bloomberg
