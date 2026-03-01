# Day 1137

## Difficulty

Medium

## Problem Statement

`cons(a, b)` constructs a pair, and `car(pair)` and `cdr(pair)` returns the first and last element of that pair. For example, `car(cons(3, 4))` returns `3`, and `cdr(cons(3, 4))` returns `4`.

Given this implementation of cons:

```
def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair
```

Implement `car` and `cdr`.

## Example

### Input
```
car(cons(3, 4))
cdr(cons(3, 4))
```
### Output
```
3
4
```

## Explanation

Given a closure-based implementation of cons that constructs a pair, implement car and cdr to extract the first and last elements.

## Company

Jane Street
