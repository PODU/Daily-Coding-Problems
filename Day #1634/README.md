# Day 1634

## Difficulty

Medium

## Problem Statement

Given a `start` word, an `end` word, and a dictionary of valid words, find the shortest transformation sequence from `start` to `end` such that only one letter is changed at each step of the sequence, and each transformed word exists in the dictionary. If there is no possible transformation, return null. Each word in the dictionary have the same length as `start` and `end` and is lowercase.

## Example

### Input
```
start = "dog", end = "cat", dictionary = {"dot", "dop", "dat", "cat"}
start = "dog", end = "cat", dictionary = {"dot", "tod", "dat", "dar"}
```
### Output
```
["dog", "dot", "dat", "cat"]
null
```

## Explanation

Find the shortest word ladder from a start word to an end word, changing one letter at a time using only dictionary words, or return null if impossible.

## Company

Facebook
