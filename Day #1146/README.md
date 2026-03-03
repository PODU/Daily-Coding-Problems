# Day 1146

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

Given a string of dominoes pushed left (L), right (R), or standing (.), simulate the falling and return the final resting orientation of each domino.

## Company

Microsoft
