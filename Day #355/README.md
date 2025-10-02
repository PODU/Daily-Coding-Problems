# Day 355

## Difficulty

Hard

## Problem Statement

You are given an array X of floating-point numbers x1, x2, ... xn. These can be rounded up or down to create a corresponding array Y of integers y1, y2, ... yn.

Write an algorithm that finds an appropriate Y array with the following properties:

 * The rounded sums of both arrays should be equal.
 * The absolute pairwise difference between elements is minimized. In other words, |x1- y1| + |x2- y2| + ... + |xn- yn| should be as small as possible.

## Example

### Input
```
[1.3, 2.3, 4.4]
```
### Output
```
[1, 2, 5]
```

## Explanation

Given an array of floats, round each element up or down to an integer so that the total sum is preserved and the total absolute difference between the original and rounded values is minimized. The given example yields an absolute difference of |1.3 - 1| + |2.3 - 2| + |4.4 - 5| = 1.

## Company

Airbnb
