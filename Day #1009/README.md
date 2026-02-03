# Day 1009

## Difficulty

Easy

## Problem Statement

Given a 2D board of characters and a word, find if the word exists in the grid.

The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.

## Example

### Input
```
board = [
  ['A','B','C','E'],
  ['S','F','C','S'],
  ['A','D','E','E']
]
```
### Output
```
exists(board, "ABCCED") returns true,
exists(board, "SEE") returns true,
exists(board, "ABCB") returns false.
```

## Explanation

Determine whether a given word can be formed from horizontally or vertically adjacent cells in a 2D board, without reusing any cell.

## Company

Coursera
