# Day 831

## Difficulty

Medium

## Problem Statement

Given a string `s` and a list of words `words`, where each word is the same length, find all starting indices of substrings in `s` that is a concatenation of every word in `words` exactly once.

The order of the indices does not matter.

## Example

### Input
```
s = "dogcatcatcodecatdog", words = ["cat", "dog"]
s = "barfoobazbitbyte", words = ["dog", "cat"]
```
### Output
```
[0, 13]
[]
```

## Explanation

Find all starting indices in s where a substring is an exact concatenation of every word in words. For the first example "dogcat" starts at 0 and "catdog" at 13; the second has no match.

## Company

Dropbox
