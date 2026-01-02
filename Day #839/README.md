# Day 839

## Difficulty

Easy

## Problem Statement

You are given an N by N matrix of random letters and a dictionary of words. Find the maximum number of words that can be packed on the board from the given dictionary.

A word is considered to be able to be packed on the board if:

 * It can be found in the dictionary
 * It can be constructed from untaken letters by other words found so far on the board
 * The letters are adjacent to each other (vertically and horizontally, not diagonally).

Each tile can be visited only once by any word.

## Example

### Input
```
dictionary = { 'eat', 'rain', 'in', 'rat' }
matrix =
[['e', 'a', 'n'],
 ['t', 't', 'i'],
 ['a', 'r', 'a']]
```
### Output
```
3
```

## Explanation

Given a letter grid and a dictionary, find the maximum number of dictionary words that can be packed onto the board using only adjacent, untaken tiles. In the example you can make 'eat', 'in', and 'rat'.

## Company

Google
