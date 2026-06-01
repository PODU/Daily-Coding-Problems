# Day 1594

## Difficulty

Easy

## Problem Statement

You are given given a list of rectangles represented by min and max x- and y-coordinates. Compute whether or not a pair of rectangles overlap each other. If one rectangle completely covers another, it is considered overlapping.

## Example

### Input
```
{
    "top_left": (1, 4),
    "dimensions": (3, 3) # width, height
},
{
    "top_left": (-1, 3),
    "dimensions": (2, 1)
},
{
    "top_left": (0, 5),
    "dimensions": (4, 3)
}
```
### Output
```
true as the first and third rectangle overlap each other.
```

## Explanation

Given a list of axis-aligned rectangles, determine whether any pair of them overlap (full containment counts as overlap).

## Company

Google
