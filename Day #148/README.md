# Day 148

## Difficulty

Medium

## Problem Statement

Gray code is a binary code where each successive value differ in only one bit, as well as when wrapping around. Gray code is common in hardware so that we don't see temporary spurious values during transitions.

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

Given n bits, produce a sequence of all 2^n binary values where each consecutive value (including wrap-around) differs from the previous in exactly one bit.

## Company

Apple
