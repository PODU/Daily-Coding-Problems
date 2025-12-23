# Day 784

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
target = 'FOAM'
```
### Output
```
true
```

## Explanation

Search the character grid horizontally and vertically for the target word; 'FOAM' appears in the leftmost column and 'MASS' in the last row, so both return true.

## Company

Microsoft
