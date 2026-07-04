# Day 1762

## Difficulty

Hard

## Problem Statement

You are presented with an array representing a Boolean expression. The elements are of two kinds:

- `T` and `F`, representing the values `True` and `False`.
- `&`, `|`, and `^`, representing the bitwise operators for `AND`, `OR`, and `XOR`.

Determine the number of ways to group the array elements using parentheses so that the entire expression evaluates to `True`.

## Example

### Input
```
['F', '|', 'T', '&', 'T']
```
### Output
```
2
```

## Explanation

Count the number of distinct parenthesizations of a Boolean expression array (with T/F and &, |, ^ operators) that evaluate to True. The example has two valid groupings.

## Company

Quantcast
