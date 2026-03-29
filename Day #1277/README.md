# Day 1277

## Difficulty

Medium

## Problem Statement

Write a function, add_subtract, which alternately adds and subtracts curried arguments. Here are some sample operations:

```
add_subtract(7) -> 7

add_subtract(1)(2)(3) -> 1 + 2 - 3 -> 0

add_subtract(-5)(10)(3)(9) -> -5 + 10 - 3 + 9 -> 11
```

## Example

### Input
```
add_subtract(-5)(10)(3)(9)
```
### Output
```
11
```

## Explanation

Implement a curried function that takes successive single arguments and alternately adds then subtracts them, returning the running result.

## Company

Squarespace
