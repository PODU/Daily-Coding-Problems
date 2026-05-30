# Day 1581

## Difficulty

Easy

## Problem Statement

Given two rectangles on a 2D graph, return the area of their intersection. If the rectangles don't intersect, return 0.

For example, given the following rectangles:

```
{
    "top_left": (1, 4),
    "dimensions": (3, 3) # width, height
}
```

and

```
{
    "top_left": (0, 5),
    "dimensions": (4, 3) # width, height
}
```

return 6.

## Example

### Input
```
{
    "top_left": (1, 4),
    "dimensions": (3, 3) # width, height
}

{
    "top_left": (0, 5),
    "dimensions": (4, 3) # width, height
}
```
### Output
```
6
```

## Explanation

Given two rectangles on a 2D graph, compute the area of their overlap, returning 0 if they do not intersect.

## Company

Google
