# Day 1332

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
[1, 2, 5], which has an absolute difference of |1.3 - 1| + |2.3 - 2| + |4.4 - 5| = 1.
```

## Explanation

Round each float up or down to integers so that the total sum is preserved while minimizing the sum of absolute differences between the originals and their rounded values.

## Company

Airbnb
