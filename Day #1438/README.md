# Day 1438

## Difficulty

Medium

## Problem Statement

You are given a histogram consisting of rectangles of different heights. These heights are represented in an input list, such that `[1, 3, 2, 5]` corresponds to the following diagram:

```
      x
      x
  x   x
  x x x
x x x x
```

Determine the area of the largest rectangle that can be formed only from the bars of the histogram. For the diagram above, for example, this would be six, representing the `2 x 3` area at the bottom right.

## Example

### Input
```
[1, 3, 2, 5]
```
### Output
```
6
```

## Explanation

Given bar heights of a histogram, find the maximum area of an axis-aligned rectangle that fits entirely within the bars. For [1, 3, 2, 5] the answer is 6 (a 2-wide, 3-tall rectangle).

## Company

Square
