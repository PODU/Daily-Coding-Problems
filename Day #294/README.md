# Day 294

## Difficulty

Medium

## Problem Statement

A competitive runner would like to create a route that starts and ends at his house, with the condition that the route goes entirely uphill at first, and then entirely downhill.

Given a dictionary of places of the form `{location: elevation}`, and a dictionary mapping paths between some of these locations to their corresponding distances, find the length of the shortest route satisfying the condition above. Assume the runner's home is location `0`.

## Example

### Input
```
elevations = {0: 5, 1: 25, 2: 15, 3: 20, 4: 10}
paths = {
    (0, 1): 10,
    (0, 2): 8,
    (0, 3): 15,
    (1, 3): 12,
    (2, 4): 10,
    (3, 4): 5,
    (3, 0): 17,
    (4, 0): 10
}
```
### Output
```
The shortest valid path would be 0 -> 2 -> 4 -> 0, with a distance of 28.
```

## Explanation

Find the shortest closed route from home that goes strictly uphill then strictly downhill.

## Company

Square
