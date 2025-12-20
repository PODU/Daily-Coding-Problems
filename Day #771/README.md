# Day 771

## Difficulty

Easy

## Problem Statement

Determine whether there exists a one-to-one character mapping from one string `s1` to another `s2`.

## Example

### Input
```
s1 = abc, s2 = bcd
s1 = foo, s2 = bar
```
### Output
```
true
false
```

## Explanation

Return true for (abc, bcd) since a->b, b->c, c->d works; return false for (foo, bar) since o cannot map to two characters. Check whether a consistent one-to-one character mapping exists between two strings.

## Company

Bloomberg
