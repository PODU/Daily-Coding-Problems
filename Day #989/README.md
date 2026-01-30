# Day 989

## Difficulty

Medium

## Problem Statement

You are given an array of length `N`, where each element `i` represents the number of ways we can produce `i` units of change. For example, `[1, 0, 1, 1, 2]` would indicate that there is only one way to make `0`, `2`, or `3` units, and two ways of making `4` units.

Given such an array, determine the denominations that must be in use. In the case above, for example, there must be coins with value `2`, `3`, and `4`.

## Example

### Input
```
[1, 0, 1, 1, 2]
```
### Output
```
2, 3, 4
```

## Explanation

Given an array where each index holds the number of ways to make that amount of change, deduce which coin denominations must be in use.

## Company

Snapchat
