# Day 1030

## Difficulty

Easy

## Problem Statement

On a mysterious island there are creatures known as Quxes which come in three colors: red, green, and blue. One power of the Qux is that if two of them are standing next to each other, they can transform into a single creature of the third color.

Given N Quxes standing in a line, determine the smallest number of them remaining after any possible sequence of such transformations.

## Example

### Input
```
['R', 'G', 'B', 'G', 'B']
```
### Output
```
1

        Arrangement       |   Change
----------------------------------------
['R', 'G', 'B', 'G', 'B'] | (R, G) -> B
['B', 'B', 'G', 'B']      | (B, G) -> R
['B', 'R', 'B']           | (R, B) -> G
['B', 'G']                | (B, G) -> R
['R']                     |
```

## Explanation

Given a line of N tri-colored Quxes where two adjacent differing creatures merge into one of the third color, find the smallest possible number remaining.

## Company

Facebook
