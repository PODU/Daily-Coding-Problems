# Day 308

## Difficulty

Hard

## Problem Statement

You are presented with an array representing a Boolean expression. The elements are of two kinds:

 * T and F, representing the values True and False.
 * &, |, and ^, representing the bitwise operators for AND, OR, and XOR.

Determine the number of ways to group the array elements using parentheses so that the entire expression evaluates to True.

## Example

### Input
```
['F', '|', 'T', '&', 'T']
```
### Output
```
2
(the two acceptable groupings are (F | T) & T and F | (T & T))
```

## Explanation

Count the number of ways to parenthesize a boolean expression of T/F values and &, |, ^ operators so that it evaluates to True.

## Company

Quantcast
