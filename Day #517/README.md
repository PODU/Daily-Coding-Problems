# Day 517

## Difficulty

Easy

## Problem Statement

Given two singly linked lists that intersect at some point, find the intersecting node. The lists are non-cyclical.

For example, given A = 3 -> 7 -> 8 -> 10 and B = 99 -> 1 -> 8 -> 10, return the node with value 8.

In this example, assume nodes with the same value are the exact same node objects.

Do this in O(M + N) time (where M and N are the lengths of the lists) and constant space.

## Example

### Input
```
A = 3 -> 7 -> 8 -> 10
B = 99 -> 1 -> 8 -> 10
```
### Output
```
The node with value 8
```

## Explanation

Find the node where two non-cyclical linked lists merge in O(M + N) time and constant space, e.g. by advancing two pointers that switch lists after reaching the end.

## Company

Google
