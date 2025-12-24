# Day 792

## Difficulty

Hard

## Problem Statement

You have `N` stones in a row, and would like to create from them a pyramid. This pyramid should be constructed such that the height of each stone increases by one until reaching the tallest stone, after which the heights decrease by one. In addition, the start and end stones of the pyramid should each be one stone high.

You can change the height of any stone by paying a cost of `1` unit to lower its height by `1`, as many times as necessary. Given this information, determine the lowest cost method to produce this pyramid.

## Example

### Input
```
[1, 1, 3, 3, 2, 1]
```
### Output
```
2 (resulting in [0, 1, 2, 3, 2, 1])
```

## Explanation

Find the minimum total cost (only lowering heights) to reshape the row of stones into a valid pyramid; for [1, 1, 3, 3, 2, 1] the optimal cost is 2, giving [0, 1, 2, 3, 2, 1].

## Company

Uber
