# Day 1740

## Difficulty

Hard

## Problem Statement

Given a string, return whether it represents a number. Here are the different kinds of numbers:

- "10", a positive integer
- "-10", a negative integer
- "10.1", a positive real number
- "-10.1", a negative real number
- "1e5", a number in scientific notation

And here are examples of non-numbers:

- "a"
- "x 1"
- "a -2"
- "-"

## Example

### Input
```
"10", "-10", "10.1", "-10.1", "1e5" (numbers); "a", "x 1", "a -2", "-" (non-numbers)
```
### Output
```
true for numbers; false for non-numbers
```

## Explanation

Determine whether a given string represents a valid number, including integers, negatives, reals, and scientific notation.

## Company

LinkedIn
