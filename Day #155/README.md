# Day 155

## Difficulty

Medium

## Problem Statement

Given a list of elements, find the majority element, which appears more than half the time (> floor(len(lst) / 2.0)).

You can assume that such element exists.

## Example

### Input
```
[1, 2, 1, 1, 3, 4, 0]
```
### Output
```
1
```

## Explanation

Use the Boyer-Moore voting algorithm to find the element that appears more than half the time in a single pass.

## Company

MongoDB
