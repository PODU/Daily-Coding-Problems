# Day 1323

## Difficulty

Hard

## Problem Statement

You have N stones in a row, and would like to create from them a pyramid. This pyramid should be constructed such that the height of each stone increases by one until reaching the tallest stone, after which the heights decrease by one. In addition, the start and end stones of the pyramid should each be one stone high.

You can change the height of any stone by paying a cost of 1 unit to lower its height by 1, as many times as necessary. Given this information, determine the lowest cost method to produce this pyramid.

## Example

### Input
```
[1, 1, 3, 3, 2, 1]
```
### Output
```
The optimal solution is to pay 2 to create [0, 1, 2, 3, 2, 1].
```

## Explanation

Given a row of stones, find the minimum total cost (only lowering heights) to reshape them into a valid pyramid that rises by one then falls by one with unit-high ends.

## Company

Uber
