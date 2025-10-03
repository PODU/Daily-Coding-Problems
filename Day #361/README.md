# Day 361

## Difficulty

Medium

## Problem Statement

Mastermind is a two-player game in which the first player attempts to guess the secret code of the second. In this version, the code may be any six-digit number with all distinct digits.

Each turn the first player guesses some number, and the second player responds by saying how many digits in this number correctly matched their location in the secret code. For example, if the secret code were 123456, then a guess of 175286 would score two, since 1 and 6 were correctly placed.

Write an algorithm which, given a sequence of guesses and their scores, determines whether there exists some secret code that could have produced them.

## Example

### Input
```
{175286: 2, 293416: 3, 654321: 0}
{123456: 4, 345678: 4, 567890: 4}
```
### Output
```
True
False
```

## Explanation

Given a set of Mastermind guesses (six-digit numbers with distinct digits) mapped to their match scores, determine whether any secret code could have produced all of them. The first example returns True (code 123456), the second returns False.

## Company

Facebook
