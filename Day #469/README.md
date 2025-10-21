# Day 469

## Difficulty

Medium

## Problem Statement

Mastermind is a two-player game in which the first player attempts to guess the secret code of the second. In this version, the code may be any six-digit number with all distinct digits.

Each turn the first player guesses some number, and the second player responds by saying how many digits in this number correctly matched their location in the secret code. For example, if the secret code were `123456`, then a guess of `175286` would score two, since `1` and `6` were correctly placed.

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

Given guesses mapped to their match-scores, determine whether any valid six-digit distinct-digit secret code is consistent with all of them. The first example matches code 123456 (True); the second is impossible (False).

## Company

Facebook
