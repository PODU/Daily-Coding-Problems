# Day 376

## Difficulty

Easy

## Problem Statement

You are writing an AI for a 2D map game. You are somewhere in a 2D grid, and there are coins strewn about over the map.

Given the position of all the coins and your current position, find the closest coin to you in terms of Manhattan distance. That is, you can move around up, down, left, and right, but not diagonally. If there are multiple possible closest coins, return any of them.

For example, given the following map, where you are `x`, coins are `o`, and empty spaces are `.` (top left is 0, 0):

```
---------------------
| . | . | x | . | o |
---------------------
| o | . | . | . | . |
---------------------
| o | . | . | . | o |
---------------------
| . | . | o | . | . |
---------------------
```

return `(0, 4)`, since that coin is closest.

## Example

### Input
```
Our position: (0, 2)
Coins: [(0, 4), (1, 0), (2, 0), (3, 2)]
```
### Output
```
(0, 4)
```

## Explanation

Given your position and a list of coin positions on a grid, return the coin with the smallest Manhattan distance to you.

## Company

Google
