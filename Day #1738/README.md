# Day 1738

## Difficulty

Hard

## Problem Statement

In a directed graph, each node is assigned an uppercase letter. We define a path's value as the number of most frequently-occurring letter along that path. For example, if a path in the graph goes through "ABACA", the value of the path is 3, since there are 3 occurrences of 'A' on the path.

Given a graph with `n` nodes and `m` directed edges, return the largest value path of the graph. If the largest value is infinite, then return null.

The graph is represented with a string and an edge list. The `i`-th character represents the uppercase letter of the `i`-th node. Each tuple in the edge list `(i, j)` means there is a directed edge from the `i`-th node to the `j`-th node. Self-edges are possible, as well as multi-edges.

## Example

### Input
```
ABACA

[(0, 1),
 (0, 2),
 (2, 3),
 (3, 4)]
```
### Output
```
3
```

## Explanation

For the first graph, the maximum value is 3 using the path of vertices [0, 2, 3, 4], (A, A, C, A). For the graph "A" with edge [(0, 0)], it should return null since there is an infinite loop. Find the path whose most frequent letter count is largest, returning null if a cycle makes it infinite.

## Company

Google
