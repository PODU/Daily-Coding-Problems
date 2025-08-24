# Day 170

## Difficulty

Medium

## Problem Statement

Given a `start` word, an `end` word, and a dictionary of valid words, find the shortest transformation sequence from `start` to `end` such that only one letter is changed at each step of the sequence, and each transformed word exists in the dictionary. If there is no possible transformation, return null. Each word in the dictionary have the same length as `start` and `end` and is lowercase.

For example, given `start = "dog"`, `end = "cat"`, and `dictionary = {"dot", "dop", "dat", "cat"}`, return `["dog", "dot", "dat", "cat"]`.

Given `start = "dog"`, `end = "cat"`, and `dictionary = {"dot", "tod", "dat", "dar"}`, return null as there is no possible transformation from `dog` to `cat`.

## Example

### Input
```
start = "dog", end = "cat", dictionary = {"dot", "dop", "dat", "cat"}
```
### Output
```
["dog", "dot", "dat", "cat"]
```

## Explanation

Find the shortest word-ladder transformation from start to end changing one letter at a time, with each intermediate word in the dictionary.

## Company

Facebook
