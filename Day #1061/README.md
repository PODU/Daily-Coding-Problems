# Day 1061

## Difficulty

Easy

## Problem Statement

You are given an M by N matrix consisting of booleans that represents a board. Each True boolean represents a wall. Each False boolean represents a tile you can walk on.

Given this matrix, a start coordinate, and an end coordinate, return the minimum number of steps required to reach the end coordinate from the start. If there is no possible path, then return null. You can move up, left, down, and right. You cannot move through walls. You cannot wrap around the edges of the board.

## Example

### Input
```
[[f, f, f, f],
[t, t, f, t],
[f, f, f, f],
[f, f, f, f]]
start = (3, 0), end = (0, 0)
```
### Output
```
7
```

## Explanation

Find the shortest path length on a grid with walls between a start and end coordinate, moving in four directions, or null if unreachable.

## Company

Google
