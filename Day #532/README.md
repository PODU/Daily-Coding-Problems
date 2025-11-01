# Day 532

## Difficulty

Medium

## Problem Statement

On our special chessboard, two bishops attack each other if they share the same diagonal. This includes bishops that have another bishop located between them, i.e. bishops can attack through pieces.

You are given N bishops, represented as (row, column) tuples on a M by M chessboard. Write a function to count the number of pairs of bishops that attack each other. The ordering of the pair doesn't matter: (1, 2) is considered the same as (2, 1).

## Example

### Input
```
M = 5, bishops = [(0, 0), (1, 2), (2, 2), (4, 0)]

[b 0 0 0 0]
[0 0 b 0 0]
[0 0 b 0 0]
[0 0 0 0 0]
[b 0 0 0 0]
```
### Output
```
2
```

## Explanation

Count pairs of bishops sharing a diagonal (attacking each other) on an M by M board, by grouping bishops along the two diagonal directions.

## Company

Google
