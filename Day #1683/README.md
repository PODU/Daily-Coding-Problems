# Day 1683

## Difficulty

Easy

## Problem Statement

Given a 2D matrix of characters and a target word, write a function that returns whether the word can be found in the matrix by going left-to-right, or up-to-down.

## Example

### Input
```
[['F', 'A', 'C', 'I'],
 ['O', 'B', 'Q', 'P'],
 ['A', 'N', 'O', 'B'],
 ['M', 'A', 'S', 'S']]

target word 'FOAM'
target word 'MASS'
```
### Output
```
'FOAM' -> true (leftmost column)
'MASS' -> true (last row)
```

## Explanation

Determine whether a target word appears in a character matrix reading either left-to-right along a row or top-to-bottom down a column.

## Company

Microsoft
