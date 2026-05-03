# Day 1464

## Difficulty

Medium

## Problem Statement

Given an array of integers in which two elements appear exactly once and all other elements appear exactly twice, find the two elements that appear only once.

Follow-up: Can you do this in linear time and constant space?

## Example

### Input
```
[2, 4, 6, 8, 10, 2, 6, 10]
```
### Output
```
4 and 8
```

## Explanation

Identify the two non-repeating elements; XOR all values then split by a differing bit to find each, achieving linear time and constant space.

## Company

Facebook
