# Day 990

## Difficulty

Hard

## Problem Statement

There exists a staircase with N steps, and you can climb up either 1 or 2 steps at a time. Given N, write a function that returns the number of unique ways you can climb the staircase. The order of the steps matters.

What if, instead of being able to climb 1 or 2 steps at a time, you could climb any number from a set of positive integers X? For example, if X = {1, 3, 5}, you could climb 1, 3, or 5 steps at a time.

## Example

### Input
```
N = 4
```
### Output
```
5 unique ways:
 * 1, 1, 1, 1
 * 2, 1, 1
 * 1, 2, 1
 * 1, 1, 2
 * 2, 2
```

## Explanation

Count the number of distinct ordered ways to climb a staircase of N steps using allowed step sizes (1 or 2, then generalized to any set X of positive integers).

## Company

Amazon
