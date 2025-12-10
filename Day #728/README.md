# Day 728

## Difficulty

Hard

## Problem Statement

A teacher must divide a class of students into two teams to play dodgeball. Unfortunately, not all the kids get along, and several refuse to be put on the same team as that of their enemies.

Given an adjacency list of students and their enemies, write an algorithm that finds a satisfactory pair of teams, or returns `False` if none exists.

## Example

### Input
```
students = {
    0: [3],
    1: [2],
    2: [1, 4],
    3: [0, 4, 5],
    4: [2, 3],
    5: [3]
}

students = {
    0: [3],
    1: [2],
    2: [1, 3, 4],
    3: [0, 2, 4, 5],
    4: [2, 3],
    5: [3]
}
```
### Output
```
{0, 1, 4, 5} and {2, 3}

False
```

## Explanation

Given an enemy adjacency list, two-color (bipartition) the graph so no two enemies share a team, or return False if the graph is not bipartite.

## Company

Twitter
