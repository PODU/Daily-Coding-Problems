# Day 1667

## Difficulty

Hard

## Problem Statement

We're given a hashmap associating each `courseId` key with a list of `courseIds` values, which represents that the prerequisites of `courseId` are `courseIds`. Return a sorted ordering of courses such that we can finish all courses.

Return null if there is no such ordering.

## Example

### Input
```
{'CSC300': ['CSC100', 'CSC200'], 'CSC200': ['CSC100'], 'CSC100': []}
```
### Output
```
['CSC100', 'CSC200', 'CSCS300']
```

## Explanation

Perform a topological sort over a course prerequisite graph to produce a valid completion order, returning null if a cycle makes ordering impossible.

## Company

Airbnb
