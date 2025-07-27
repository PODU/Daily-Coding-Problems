# Day 35

## Difficulty

Hard

## Problem Statement

Given an array of strictly the characters 'R', 'G', and 'B', segregate the values of the array so that all the Rs come first, the Gs come second, and the Bs come last. You can only swap elements of the array.

Do this in linear time and in-place.

For example, given the array ['G', 'B', 'R', 'R', 'B', 'R', 'G'], it should become ['R', 'R', 'R', 'G', 'G', 'B', 'B'].

## Example

### Input
```
['G', 'B', 'R', 'R', 'B', 'R', 'G']
```
### Output
```
['R', 'R', 'R', 'G', 'G', 'B', 'B']
```

## Explanation

Sort an array of only 'R', 'G', and 'B' so all Rs precede Gs precede Bs, using in-place swaps in linear time (Dutch national flag).

## Company

Google
