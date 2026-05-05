# Day 1475

## Difficulty

Medium

## Problem Statement

Implement an autocomplete system. That is, given a query string `s` and a set of all possible query strings, return all strings in the set that have s as a prefix.

Hint: Try preprocessing the dictionary into a more efficient data structure to speed up queries.

## Example

### Input
```
de, [dog, deer, deal]
```
### Output
```
[deer, deal]
```

## Explanation

Return every string in the set that begins with the given prefix; a trie is a natural preprocessing structure.

## Company

Twitter
