# Day 1572

## Difficulty

Easy

## Problem Statement

Given a mapping of digits to letters (as in a phone number), and a digit string, return all possible letters the number could represent. You can assume each valid number in the mapping is a single digit.

For example if {“2”: [“a”, “b”, “c”], 3: [“d”, “e”, “f”], …} then “23” should return [“ad”, “ae”, “af”, “bd”, “be”, “bf”, “cd”, “ce”, “cf"].

## Example

### Input
```
{"2": ["a", "b", "c"], 3: ["d", "e", "f"], ...}, "23"
```
### Output
```
["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
```

## Explanation

Given a digit-to-letters mapping (like a phone keypad) and a digit string, return every possible letter combination the digits could represent.

## Company

Yelp
