# Day 1423

## Difficulty

Medium

## Problem Statement

You are given an array of nonnegative integers. Let's say you start at the
beginning of the array and are trying to advance to the end. You can advance at
most, the number of steps that you're currently on. Determine whether you can
get to the end of the array.

## Example

### Input
```
[1, 3, 1, 2, 0, 1]  ->  true
[1, 2, 1, 0, 0]     ->  false
```
### Output
```
[1, 3, 1, 2, 0, 1] returns true (0 -> 1 -> 3 -> 5)
[1, 2, 1, 0, 0] returns false
```

## Explanation

Given an array of nonnegative integers where each value is the max steps you can advance, determine whether you can reach the end starting from the beginning.

## Company

Google
