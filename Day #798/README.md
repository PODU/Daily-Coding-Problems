# Day 798

## Difficulty

Medium

## Problem Statement

You are given a set of synonyms, such as `(big, large)` and `(eat, consume)`. Using this set, determine if two sentences with the same number of words are equivalent.

For example, the following two sentences are equivalent:

 * "He wants to eat food."
 * "He wants to consume food."

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
True (equivalent)
```

## Explanation

Given a set of synonym pairs, decide whether two equal-length sentences mean the same thing by matching each word position (directly or via synonyms).

## Company

Google
