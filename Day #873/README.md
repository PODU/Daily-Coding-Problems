# Day 873

## Difficulty

Hard

## Problem Statement

Given an arithmetic expression in Reverse Polish Notation, write a program to evaluate it.

The expression is given as a list of numbers and operands. For example: `[5, 3, '+']` should return `5 + 3 = 8`.

You can assume the given expression is always valid.

## Example

### Input
```
[15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-']
```
### Output
```
5
```

## Explanation

Evaluate a Reverse Polish Notation expression; the example is equivalent to ((15 / (7 - (1 + 1))) * 3) - (2 + (1 + 1)) = 5.

## Company

Jane Street
