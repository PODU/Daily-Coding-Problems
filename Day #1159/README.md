# Day 1159

## Difficulty

Easy

## Problem Statement

Given an even number (greater than 2), return two prime numbers whose sum will be equal to the given number.

A solution will always exist. See Goldbach's conjecture.

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

Given an even number greater than 2, find two primes that sum to it (Goldbach's conjecture), returning the lexicographically smallest pair.

## Company

Alibaba
