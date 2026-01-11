# Day 881

## Difficulty

Medium

## Problem Statement

Implement a 2D iterator class. It will be initialized with an array of arrays, and should implement the following methods:

 * next(): returns the next element in the array of arrays. If there are no more elements, raise an exception.
 * has_next(): returns whether or not the iterator still has elements left.

Do not use flatten or otherwise clone the arrays. Some of the arrays can be empty.

## Example

### Input
```
[[1, 2], [3], [], [4, 5, 6]]
```
### Output
```
1, 2, 3, 4, 5, 6
```

## Explanation

Build a 2D iterator with next() and has_next() over an array of arrays without flattening or cloning the input.

## Company

Uber
