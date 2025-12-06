# Day 704

## Difficulty

Medium

## Problem Statement

One way to unlock an Android phone is through a pattern of swipes across a `1-9` keypad.

For a pattern to be valid, it must satisfy the following:

 * All of its keys must be distinct.
 * It must not connect two keys by jumping over a third key, unless that key has already been used.

For example, `4 - 2 - 1 - 7` is a valid pattern, whereas `2 - 1 - 7` is not.

Find the total number of valid unlock patterns of length `N`, where `1 <= N <= 9`.

## Example

### Input
```
N (1 <= N <= 9)
```
### Output
```
Total number of valid unlock patterns of length N
```

## Explanation

Count the valid Android unlock patterns of a given length on a 3x3 keypad, where keys must be distinct and a swipe cannot jump over an unused key.

## Company

Uber
