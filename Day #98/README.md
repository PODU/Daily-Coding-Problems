# Day 98

## Difficulty

Easy

## Problem Statement

Given a 2D board of characters and a word, find if the word exists in the grid.

The word can be constructed from letters of sequentially adjacent cell, where "adjacent" cells are those horizontally or vertically neighboring. The same letter cell may not be used more than once.

## Example

### Input
```
[
  ['A','B','C','E'],
  ['S','F','C','S'],
  ['A','D','E','E']
]
exists(board, "ABCCED")
exists(board, "SEE")
exists(board, "ABCB")
```
### Output
```
true
true
false
```

## Explanation

Determine whether a word can be traced through adjacent cells in a character grid without reusing a cell.

## Company

Coursera
