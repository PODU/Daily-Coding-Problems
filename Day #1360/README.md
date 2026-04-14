# Day 1360

## Difficulty

Medium

## Problem Statement

One way to unlock an Android phone is through a pattern of swipes across a 1-9 keypad.

For a pattern to be valid, it must satisfy the following:

 * All of its keys must be distinct.
 * It must not connect two keys by jumping over a third key, unless that key has already been used.

For example, 4 - 2 - 1 - 7 is a valid pattern, whereas 2 - 1 - 7 is not.

Find the total number of valid unlock patterns of length N, where 1 <= N <= 9.

## Example

### Input
```
N (1 <= N <= 9); e.g. valid: 4 - 2 - 1 - 7, invalid: 2 - 1 - 7
```
### Output
```
total number of valid unlock patterns of length N
```

## Explanation

Count the number of valid Android unlock patterns of a given length N, respecting the distinct-key and no-jumping-over-unused-key rules.

## Company

Uber
