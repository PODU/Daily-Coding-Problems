# Day 510

## Difficulty

Hard

## Problem Statement

Given a list of words, find all pairs of unique indices such that the concatenation of the two words is a palindrome.

For example, given the list `["code", "edoc", "da", "d"]`, return `[(0, 1), (1, 0), (2, 3)]`.

## Example

### Input
```
["code", "edoc", "da", "d"]
```
### Output
```
[(0, 1), (1, 0), (2, 3)]
```

## Explanation

Find all index pairs whose concatenated words form a palindrome, efficiently by checking prefix/suffix palindrome conditions against reversed words stored in a hash map.

## Company

Airbnb
