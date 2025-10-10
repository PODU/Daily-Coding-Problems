# Day 406

## Difficulty

Hard

## Problem Statement

You are presented with an array representing a Boolean expression. The elements are of two kinds:

 * T and F, representing the values True and False.
 * &, |, and ^, representing the bitwise operators for AND, OR, and XOR.

Determine the number of ways to group the array elements using parentheses so that the entire expression evaluates to True.

For example, suppose the input is ['F', '|', 'T', '&', 'T']. In this case, there are two acceptable groupings: (F | T) & T and F | (T & T).

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

Count the number of parenthesizations of a Boolean expression that make it evaluate to True.

## Company

Quantcast
