# Day 264

## Difficulty

Hard

## Problem Statement

Given a set of characters `C` and an integer `k`, a De Bruijn sequence is a cyclic sequence in which every possible `k`-length string of characters in `C` occurs exactly once.

Create an algorithm that finds a De Bruijn sequence.

## Example

### Input
```
C = {0, 1} and k = 3
```
### Output
```
00010111
```

## Explanation

For C = {0, 1} and k = 3, the sequence should contain the substrings {'000', '001', '010', '011', '100', '101', '110', '111'}, and one possible solution would be 00010111. Build a cyclic sequence over the given character set in which every length-k string appears exactly once.

## Company

LinkedIn
