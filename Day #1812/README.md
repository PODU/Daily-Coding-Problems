# Day 1812

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
```
### Output
```
{0, 1, 4, 5} and {2, 3}
```

## Explanation

Given an enemy adjacency list, split the students into two teams such that no student shares a team with an enemy (a graph 2-coloring / bipartite check), or return False if impossible.

## Company

Twitter
