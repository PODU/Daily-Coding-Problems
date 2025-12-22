# Day 779

## Difficulty

Medium

## Problem Statement

You are given `N` identical eggs and access to a building with `k` floors. Your task is to find the lowest floor that will cause an egg to break, if dropped from that floor. Once an egg breaks, it cannot be dropped again. If an egg breaks when dropped from the `xth` floor, you can assume it will also break when dropped from any floor greater than `x`.

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

With one egg and 5 floors, you must drop from each floor in order starting from the first until it breaks, requiring 5 drops in the worst case.

## Company

Goldman Sachs
