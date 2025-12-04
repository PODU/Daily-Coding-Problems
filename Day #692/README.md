# Day 692

## Difficulty

Medium

## Problem Statement

Implement an autocomplete system. That is, given a query string `s` and a set of all possible query strings, return all strings in the set that have s as a prefix.

Hint: Try preprocessing the dictionary into a more efficient data structure to speed up queries.

## Example

### Input
```
s = "de", strings = [dog, deer, deal]
```
### Output
```
[deer, deal]
```

## Explanation

Return all strings from a given set that begin with the query string s, ideally by preprocessing the dictionary (e.g. into a trie) for fast lookups.

## Company

Twitter
