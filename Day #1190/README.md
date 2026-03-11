# Day 1190

## Difficulty

Medium

## Problem Statement

A network consists of nodes labeled `0` to `N`. You are given a list of edges `(a, b, t)`, describing the time `t` it takes for a message to be sent from node `a` to node `b`. Whenever a node receives a message, it immediately passes the message on to a neighboring node, if possible.

Assuming all nodes are connected, determine how long it will take for every node to receive a message that begins at node `0`.

## Example

### Input
```
N = 5
edges = [
    (0, 1, 5),
    (0, 2, 3),
    (0, 5, 4),
    (1, 3, 8),
    (2, 3, 1),
    (3, 5, 10),
    (3, 4, 5)
]
```
### Output
```
9
```

## Explanation

Given a network with weighted edges representing message travel times, find the time for a message starting at node 0 to reach every node (the shortest-path propagation time).

## Company

Twitter
