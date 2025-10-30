# Day 515

## Difficulty

Medium

## Problem Statement

Given a linked list of numbers and a pivot `k`, partition the linked list so that all nodes less than `k` come before nodes greater than or equal to `k`.

For example, given the linked list `5 -> 1 -> 8 -> 0 -> 3` and `k = 3`, the solution could be `1 -> 0 -> 5 -> 8 -> 3`.

## Example

### Input
```
5 -> 1 -> 8 -> 0 -> 3, k = 3
```
### Output
```
1 -> 0 -> 5 -> 8 -> 3
```

## Explanation

Partition a linked list around a pivot value k so nodes less than k precede nodes greater than or equal to k, by splitting into two sublists and concatenating them.

## Company

LinkedIn
