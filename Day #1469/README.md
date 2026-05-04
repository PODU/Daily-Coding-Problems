# Day 1469

## Difficulty

Hard

## Problem Statement

You are presented with an `8` by `8` matrix representing the positions of pieces on a chess board. The only pieces on the board are the black king and various white pieces. Given this matrix, determine whether the king is in check.

For details on how each piece moves, see [here](https://en.wikipedia.org/wiki/Chess_piece#Moves_of_the_pieces).

## Example

### Input
```
...K....
........
.B......
......P.
.......R
..N.....
........
.....Q..
```
### Output
```
True
```

## Explanation

Check whether any white piece attacks the black king's square; in the example the bishop attacks the king diagonally, so the answer is True.

## Company

Oracle
