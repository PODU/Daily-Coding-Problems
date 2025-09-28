# Day 345

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
synonyms = (big, large), (eat, consume)
sentence 1 = "He wants to eat food."
sentence 2 = "He wants to consume food."
```
### Output
```
equivalent
```

## Explanation

Given a set of synonym pairs, determine whether two equal-length sentences are equivalent word-for-word under those synonyms, with a follow-up assuming synonym relations are transitive.

## Company

Google
