# Day 1522

## Difficulty

Medium

## Problem Statement

Given a string `s` and a list of words `words`, where each word is the same length, find all starting indices of substrings in `s` that is a concatenation of every word in `words` exactly once.

For example, given s = "dogcatcatcodecatdog" and words = ["cat", "dog"], return [0, 13], since "dogcat" starts at index 0 and "catdog" starts at index 13.

Given s = "barfoobazbitbyte" and words = ["dog", "cat"], return [] since there are no substrings composed of "dog" and "cat" in `s`.

The order of the indices does not matter.

## Example

### Input
```
s = "dogcatcatcodecatdog", words = ["cat", "dog"]
```
### Output
```
[0, 13]
```

## Explanation

Find all starting indices in s where a substring is a concatenation of every word in the list exactly once.

## Company

Dropbox
