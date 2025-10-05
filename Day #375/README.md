# Day 375

## Difficulty

Medium

## Problem Statement

The h-index is a metric used to measure the impact and productivity of a scientist or researcher.

A scientist has index h if h of their N papers have **at least** h citations each, and the other N - h papers have no more than h citations each. If there are multiple possible values for h, the maximum value is used.

Given an array of natural numbers, with each value representing the number of citations of a researcher's paper, return the h-index of that researcher.

## Example

### Input
```
[4, 0, 0, 2, 3]
```
### Output
```
2
```

## Explanation

Compute the h-index from an array of citation counts: the largest h such that at least h papers each have at least h citations.

## Company

Google
