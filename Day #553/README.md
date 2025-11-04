# Day 553

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

abcdef

zyx
wvu
tsr
```
### Output
```
1

0

3
```

## Explanation

Count the minimum number of columns to delete so that every remaining column is non-decreasing (lexicographically ordered) from top to bottom.

## Company

Google
