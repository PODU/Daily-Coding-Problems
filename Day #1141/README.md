# Day 1141

## Difficulty

Medium

## Problem Statement

There are `M` people sitting in a row of `N` seats, where `M < N`. Your task is to redistribute people such that there are no gaps between any of them, while keeping overall movement to a minimum.

Given an input such as the one above, return the lowest possible cost of moving people to remove all gaps.

## Example

### Input
```
[0, 1, 1, 0, 1, 0, 0, 0, 1]
```
### Output
```
5
```

## Explanation

Given a row of seats where 1 is a person and 0 is empty, compute the minimum total movement cost (sum of absolute distances) to pack all people together with no gaps.

## Company

Walmart Labs
