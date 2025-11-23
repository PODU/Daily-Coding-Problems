# Day 645

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

Target words: 'FOAM', 'MASS'
```
### Output
```
true (for 'FOAM')
true (for 'MASS')
```

## Explanation

Search a character grid for a target word horizontally (left-to-right) or vertically (up-to-down); 'FOAM' is the leftmost column and 'MASS' is the last row.

## Company

Microsoft
