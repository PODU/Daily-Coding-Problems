# Day 1076

## Difficulty

Medium

## Problem Statement

Write a function, `add_subtract`, which alternately adds and subtracts curried arguments. Here are some sample operations:

## Example

### Input
```
add_subtract(7)
add_subtract(1)(2)(3)
add_subtract(-5)(10)(3)(9)
```
### Output
```
add_subtract(7) -> 7
add_subtract(1)(2)(3) -> 1 + 2 - 3 -> 0
add_subtract(-5)(10)(3)(9) -> -5 + 10 - 3 + 9 -> 11
```

## Explanation

Implement a curried function that alternately adds and subtracts its successively supplied arguments.

## Company

Squarespace
