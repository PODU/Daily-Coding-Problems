# Day 797

## Difficulty

Easy

## Problem Statement

Given an even number (greater than 2), return two prime numbers whose sum will be equal to the given number.

A solution will always exist. See Goldbach's conjecture.

If there are more than one solution possible, return the lexicographically smaller solution.

If [a, b] is one solution with a <= b, and [c, d] is another solution with c <= d, then

[a, b] < [c, d]

If a < c OR a==c AND b < d.

## Example

### Input
```
4
```
### Output
```
2 + 2 = 4
```

## Explanation

For an even number greater than 2, find two primes that sum to it, returning the lexicographically smallest such pair.

## Company

Alibaba
