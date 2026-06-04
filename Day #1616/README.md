# Day 1616

## Difficulty

Easy

## Problem Statement

Given an even number (greater than 2), return two prime numbers whose sum will be equal to the given number.

A solution will always exist. See [Goldbach's conjecture](https://en.wikipedia.org/wiki/Goldbach%27s_conjecture).

If there are more than one solution possible, return the lexicographically smaller solution.

If [a, b] is one solution with a <= b, and [c, d] is another solution with c <= d, then

```
[a, b] < [c, d]
```

If a < c OR a==c AND b < d.

## Example

### Input
```
Input: 4
```
### Output
```
Output: 2 + 2 = 4
```

## Explanation

For an even number greater than 2, return the lexicographically smallest pair of primes that sum to it (per Goldbach's conjecture).

## Company

Alibaba
