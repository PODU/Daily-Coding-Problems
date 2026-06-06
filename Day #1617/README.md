# Day 1617

## Difficulty

Easy

## Problem Statement

You are given an string representing the initial conditions of some dominoes. Each element can take one of three values:

* `L`, meaning the domino has just been pushed to the left,
* `R`, meaning the domino has just been pushed to the right, or
* `.`, meaning the domino is standing still.

Determine the orientation of each tile when the dominoes stop falling. Note that if a domino receives a force from the left and right side simultaneously, it will remain upright.

## Example

### Input
```
.L.R....L
..R...L.L
```
### Output
```
LL.RRRLLL
..RR.LLLL
```

## Explanation

Simulate falling dominoes and return the final orientation of each tile, where a tile pushed from both sides simultaneously stays upright.

## Company

Microsoft
