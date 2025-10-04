# Day 367

## Difficulty

Medium

## Problem Statement

Given two sorted iterators, merge it into one iterator.

Bonus: Make it work without pulling in the contents of the iterators in memory.

## Example

### Input
```
foo = iter([5, 10, 15])
bar = iter([3, 8, 9])

for num in merge_iterators(foo, bar):
    print(num)
```
### Output
```
3
5
8
9
10
15
```

## Explanation

Merge two already-sorted iterators into a single sorted iterator, ideally lazily without loading all elements into memory.

## Company

Two Sigma
