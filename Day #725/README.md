# Day 725

## Difficulty

Easy

## Problem Statement

Consider the following scenario: there are `N` mice and `N` holes placed at integer points along a line. Given this, find a method that maps mice to holes such that the largest number of steps any mouse takes is minimized.

Each move consists of moving one mouse one unit to the left or right, and only one mouse can fit inside each hole.

## Example

### Input
```
mice = [1, 4, 9, 15], holes = [10, -5, 0, 16]
```
### Output
```
6
```

## Explanation

Assign mice to holes on a line so that the maximum distance any single mouse travels is minimized; sorting both lists and pairing in order achieves this.

## Company

Amazon
