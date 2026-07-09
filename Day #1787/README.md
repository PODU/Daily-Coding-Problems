# Day 1787

## Difficulty

Easy

## Problem Statement

There are N prisoners standing in a circle, waiting to be executed. The executions are carried out starting with the kth person, and removing every successive kth person going clockwise until there is no one left.

Given N and k, write an algorithm to determine where a prisoner should stand in order to be the last survivor.

Bonus: Find an O(log N) solution if k = 2.

## Example

### Input
```
N = 5, k = 2
```
### Output
```
3
```

## Explanation

This is the Josephus problem: find the position of the last survivor when every kth person in a circle of N is eliminated.

## Company

Bloomberg
