# Day 682

## Difficulty

Medium

## Problem Statement

Write a function, `add_subtract`, which alternately adds and subtracts curried arguments. Here are some sample operations:

```python
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

Implement a curried function add_subtract that takes successive single arguments and alternately adds and subtracts them.

## Company

Squarespace
