# Day 1035

## Difficulty

Easy

## Problem Statement

MegaCorp wants to give bonuses to its employees based on how many lines of codes they have written. They would like to give the smallest positive amount to each worker consistent with the constraint that if a developer has written more lines of code than their neighbor, they should receive more money.

Given an array representing a line of seats of employees at MegaCorp, determine how much each one should get paid.

## Example

### Input
```
[10, 40, 200, 1000, 60, 30]
```
### Output
```
[1, 2, 3, 4, 2, 1]
```

## Explanation

Assign each employee the smallest positive bonus so that anyone who wrote more lines than a neighbor gets paid more than that neighbor.

## Company

Atlassian
