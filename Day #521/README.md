# Day 521

## Difficulty

Medium

## Problem Statement

Given a string and a number of lines `k`, print the string in zigzag form. In zigzag, characters are printed out diagonally from top left to bottom right until reaching the `k`th line, then back up to top right, and so on.

For example, given the sentence `"thisisazigzag"` and `k = 4`, you should print:

```
t     a     g
 h   s z   a
  i i   i z
   s     g
```

## Example

### Input
```
"thisisazigzag", k = 4
```
### Output
```
t     a     g
 h   s z   a
  i i   i z
   s     g
```

## Explanation

Arrange the characters of a string into a zigzag pattern across k lines, descending then ascending diagonally, and print each row.

## Company

PayPal
