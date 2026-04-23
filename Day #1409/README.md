# Day 1409

## Difficulty

Easy

## Problem Statement

The transitive closure of a graph is a measure of which vertices are reachable from other vertices. It can be represented as a matrix `M`, where `M[i][j] == 1` if there is a path between vertices `i` and `j`, and otherwise `0`.

Given a graph, find its transitive closure.

## Example

### Input
```
graph = [
    [0, 1, 3],
    [1, 2],
    [2],
    [3]
]
```
### Output
```
[1, 1, 1, 1]
[0, 1, 1, 0]
[0, 0, 1, 0]
[0, 0, 0, 1]
```

## Explanation

Compute the transitive closure of a directed graph, producing a reachability matrix where M[i][j] is 1 if vertex j is reachable from vertex i.

## Company

Microsoft
