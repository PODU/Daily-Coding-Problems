# Day 492

## Difficulty

Medium

## Problem Statement

Given an undirected graph represented as an adjacency matrix and an integer k, write a function to determine whether each vertex in the graph can be colored such that no two adjacent vertices share the same color using at most k colors.

## Example

### Input
```
graph = [[0, 1, 1],
         [1, 0, 1],
         [1, 1, 0]]  # triangle: every pair of vertices adjacent
k = 2, then k = 3
```
### Output
```
k=2 colorable: false
k=3 colorable: true
```

## Explanation

Determine whether a graph (given as an adjacency matrix) can be colored with at most k colors so that no two adjacent vertices share the same color.

## Company

Google
