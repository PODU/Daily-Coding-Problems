# Day 162

## Difficulty

Medium

## Problem Statement

Given a list of words, return the shortest unique prefix of each word.

## Example

### Input
```
dog
cat
apple
apricot
fish
```
### Output
```
d
c
app
apr
f
```

## Explanation

Build a trie recording how many words pass through each node, then for each word take the shortest prefix whose count is 1.

## Company

Square
