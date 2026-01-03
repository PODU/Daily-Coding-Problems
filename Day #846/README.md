# Day 846

## Difficulty

Medium

## Problem Statement

cons(a, b) constructs a pair, and car(pair) and cdr(pair) returns the first and last element of that pair. For example, car(cons(3, 4)) returns 3, and cdr(cons(3, 4)) returns 4.

Given this implementation of cons:

```
def cons(a, b):
    def pair(f):
        return f(a, b)
    return pair
```

Implement car and cdr.

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

Implement car and cdr accessors for a pair built with the given closure-based cons function.

## Company

Jane Street
