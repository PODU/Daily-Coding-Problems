# Day 1461

## Difficulty

Medium

## Problem Statement

You are given `N` identical eggs and access to a building with `k` floors. Your task is to find the lowest floor that will cause an egg to break, if dropped from that floor. Once an egg breaks, it cannot be dropped again. If an egg breaks when dropped from the `x`th floor, you can assume it will also break when dropped from any floor greater than `x`.

Write an algorithm that finds the minimum number of trial drops it will take, in the worst case, to identify this floor.

## Example

### Input
```
N = 1, k = 5
```
### Output
```
5
```

## Explanation

With one egg and five floors you must test every floor from the bottom up, so the answer is 5. Find the minimum number of worst-case egg drops to identify the breaking floor.

## Company

Goldman Sachs
