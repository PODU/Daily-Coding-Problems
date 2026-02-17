# Day 1088

## Difficulty

Hard

## Problem Statement

You are presented with an array representing a Boolean expression. The elements are of two kinds:

 * `T` and `F`, representing the values `True` and `False`.
 * `&`, `|`, and `^`, representing the bitwise operators for `AND`, `OR`, and `XOR`.

Determine the number of ways to group the array elements using parentheses so that the entire expression evaluates to `True`.

## Example

### Input
```
['F', '|', 'T', '&', 'T']
```
### Output
```
2

Acceptable groupings: (F | T) & T and F | (T & T).
```

## Explanation

Count the number of ways to parenthesize a boolean expression of T/F values and &, |, ^ operators so it evaluates to True.

## Company

Quantcast
