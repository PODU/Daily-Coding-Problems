# Day 159

## Difficulty

Easy

## Problem Statement

Given a string, return the first recurring character in it, or null if there is no recurring character.

## Example

### Input
```
"acbbac"
"abcdef"
```
### Output
```
"b"
null
```

## Explanation

Scan the string left to right, tracking seen characters in a set and returning the first one already seen.

## Company

Google
