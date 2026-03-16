# Day 1216

## Difficulty

Medium

## Problem Statement

You are given an N by M 2D matrix of lowercase letters. Determine the minimum number of columns that can be removed to ensure that each row is ordered from top to bottom lexicographically. That is, the letter at each column is lexicographically later as you go down each row. It does not matter whether each row itself is ordered lexicographically.

## Example

### Input
```
cba
daf
ghi
```
### Output
```
1
```

## Explanation

Given a grid of lowercase letters, count the minimum number of columns to delete so that, within every remaining column, the letters are non-decreasing from top to bottom. In the example the second column (b, a, h) breaks the ordering, so removing it (returning 1) makes every column ordered.

## Company

Google
