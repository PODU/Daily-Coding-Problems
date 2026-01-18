# Day 915

## Difficulty

Hard

## Problem Statement

You are given an array `X` of floating-point numbers x1, x2, ... xn. These can be rounded up or down to create a corresponding array `Y` of integers y1, y2, ... yn.

Write an algorithm that finds an appropriate `Y` array with the following properties:

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

Round each float up or down to integers so the total sum is preserved while minimizing the total absolute difference; for [1.3, 2.3, 4.4] the best is [1, 2, 5] with an absolute difference of 1.

## Company

Airbnb
