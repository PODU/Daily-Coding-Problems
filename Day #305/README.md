# Day 305

## Difficulty

Easy

## Problem Statement

Given a linked list, remove all consecutive nodes that sum to zero. Print out the remaining nodes.

## Example

### Input
```
3 -> 4 -> -7 -> 5 -> -6 -> 6
```
### Output
```
5
(first remove 3 -> 4 -> -7, then -6 -> 6, leaving only 5)
```

## Explanation

Repeatedly remove any run of consecutive linked-list nodes whose values sum to zero, then output the remaining nodes.

## Company

Amazon
