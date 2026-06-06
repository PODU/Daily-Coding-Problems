# Day 1623

## Difficulty

Medium

## Problem Statement

You are given a set of synonyms, such as `(big, large)` and `(eat, consume)`. Using this set, determine if two sentences with the same number of words are equivalent.

Note that the synonyms `(a, b)` and `(a, c)` do not necessarily imply `(b, c)`: consider the case of `(coach, bus)` and `(coach, teacher)`.

Follow-up: what if we can assume that `(a, b)` and `(a, c)` do in fact imply `(b, c)`?

## Example

### Input
```
"He wants to eat food."
"He wants to consume food."
```
### Output
```
The two sentences are equivalent.
```

## Explanation

Given a synonym set, decide whether two equal-length sentences are equivalent by checking that each pair of corresponding words is either identical or listed as synonyms.

## Company

Google
