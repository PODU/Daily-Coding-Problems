# Day 448

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

Sort an array of R, G, B characters in-place in linear time using swaps (Dutch national flag problem).

## Company

Google
