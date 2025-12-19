# Day 767

## Difficulty

Hard

## Problem Statement

Given a word W and a string `S`, find all starting indices in `S` which are anagrams of `W`.

## Example

### Input
```
W is "ab", S is "abxaba"
```
### Output
```
0, 3, 4
```

## Explanation

Return every index where a substring of S of length |W| is a permutation of W (sliding-window anagram search).

## Company

Google
