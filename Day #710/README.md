# Day 710

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

Find every start index where a substring of s is an exact concatenation of all words (each used once), given all words share the same length.

## Company

Dropbox
