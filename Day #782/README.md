# Day 782

## Difficulty

Easy

## Problem Statement

In academia, the h-index is a metric used to calculate the impact of a researcher's papers. It is calculated as follows:

A researcher has index `h` if at least `h` of her `N` papers have `h` citations each. If there are multiple `h` satisfying this formula, the maximum is chosen.

Given a list of paper citations of a researcher, calculate their h-index.

## Example

### Input
```
N = 5, citations = [4, 3, 0, 1, 5]
```
### Output
```
3
```

## Explanation

Compute the largest h such that at least h papers have at least h citations each; for [4, 3, 0, 1, 5] the h-index is 3.

## Company

Palantir
