# Day 506

## Difficulty

Medium

## Problem Statement

Given a linked list, rearrange the node values such that they appear in alternating `low -> high -> low -> high ...` form. For example, given `1 -> 2 -> 3 -> 4 -> 5`, you should return `1 -> 3 -> 2 -> 5 -> 4`.

## Example

### Input
```
1 -> 2 -> 3 -> 4 -> 5
```
### Output
```
1 -> 3 -> 2 -> 5 -> 4
```

## Explanation

Rearrange linked list values into an alternating low/high (zigzag) pattern by swapping adjacent nodes that violate the expected ordering in a single pass.

## Company

Fitbit
