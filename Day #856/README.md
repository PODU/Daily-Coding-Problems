# Day 856

## Difficulty

Medium

## Problem Statement

You are given a list of `(website, user)` pairs that represent users visiting websites. Come up with a program that identifies the top `k` pairs of websites with the greatest similarity.

## Example

### Input
```
k = 1
[('a', 1), ('a', 3), ('a', 5),
 ('b', 2), ('b', 6),
 ('c', 1), ('c', 2), ('c', 3), ('c', 4), ('c', 5)
 ('d', 4), ('d', 5), ('d', 6), ('d', 7),
 ('e', 1), ('e', 3), ('e': 5), ('e', 6)]
```
### Output
```
[('a', 'e')]
```

## Explanation

From a list of (website, user) visits, compute a similarity metric between websites based on shared users and return the top k most similar website pairs.

## Company

Quora
