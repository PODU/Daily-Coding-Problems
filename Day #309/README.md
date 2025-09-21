# Day 309

## Difficulty

Medium

## Problem Statement

There are M people sitting in a row of N seats, where M < N. Your task is to redistribute people such that there are no gaps between any of them, while keeping overall movement to a minimum.

We can consider the cost of a solution to be the sum of the absolute distance each person must move.

Given an input such as the one above, return the lowest possible cost of moving people to remove all gaps.

## Example

### Input
```
[0, 1, 1, 0, 1, 0, 0, 0, 1]
(0 represents an empty seat and 1 represents a person)
```
### Output
```
5
(one solution would be to place the person on the right in the fourth seat, for a cost of five)
```

## Explanation

Given a row of seats with some occupied, find the minimum total movement (sum of absolute distances) needed to pack everyone together with no gaps.

## Company

Walmart Labs
