# Day 363

## Difficulty

Medium

## Problem Statement

Write a function, `add_subtract`, which alternately adds and subtracts curried arguments.

## Example

### Input
```
add_subtract(7)
add_subtract(1)(2)(3)
add_subtract(-5)(10)(3)(9)
```
### Output
```
7
1 + 2 - 3 -> 0
-5 + 10 - 3 + 9 -> 11
```

## Explanation

Implement a curried function that takes arguments one at a time and alternately adds and subtracts them, returning the running result.

## Company

Squarespace
