# Day 526

## Difficulty

Easy

## Problem Statement

You are given a string of length `N` and a parameter `k`. The string can be manipulated by taking one of the first `k` letters and moving it to the end.

Write a program to determine the lexicographically smallest string that can be created after an unlimited number of moves.

## Example

### Input
```
"daily", k = 1
```
### Output
```
"ailyd"
```

## Explanation

Find the lexicographically smallest string reachable by repeatedly moving one of the first k characters to the end; for k = 1 this is the smallest rotation, for k >= 2 the string can be fully sorted.

## Company

Yahoo
