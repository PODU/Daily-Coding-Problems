# Day 908

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

Count the minimum number of columns to delete so that, within every remaining column, the letters are non-decreasing from top to bottom. Removing the middle column of the example (leaving ca/df/gi) makes each column ordered, so the answer is 1.

## Company

Google
