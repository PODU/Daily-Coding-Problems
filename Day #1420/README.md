# Day 1420

## Difficulty

Medium

## Problem Statement

Given a string s and an integer k, break up the string into multiple lines such
that each line has a length of k or less. You must break it up so that words
don't break across lines. Each line has to have the maximum possible amount of
words. If there's no way to break the text up, then return null.

You can assume that there are no spaces at the ends of the string and that there
is exactly one space between each word.

## Example

### Input
```
s = "the quick brown fox jumps over the lazy dog", k = 10
```
### Output
```
["the quick", "brown fox", "jumps over", "the lazy", "dog"]
```

## Explanation

Wrap a string into lines no longer than k, never splitting words, packing the maximum words per line, returning null if impossible.

## Company

Amazon
