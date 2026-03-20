# Day 1240

## Difficulty

Medium

## Problem Statement

[Gray code](https://en.wikipedia.org/wiki/Gray_code) is a binary code where each successive value differ in only one bit, as well as when wrapping around. Gray code is common in hardware so that we don't see temporary spurious values during transitions.

Given a number of bits `n`, generate a possible gray code for it.

## Example

### Input
```
n = 2
```
### Output
```
[00, 01, 11, 10]
```

## Explanation

Generate a Gray code sequence for n bits, where consecutive values (including the wrap-around) differ by exactly one bit.

## Company

Apple
