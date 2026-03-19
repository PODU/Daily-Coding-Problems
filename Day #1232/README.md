# Day 1232

## Difficulty

Medium

## Problem Statement

You are given a 2-d matrix where each cell consists of either `/`, `\`, or an empty space. Write an algorithm that determines into how many regions the slashes divide the space.

For example, suppose the input for a three-by-six grid is the following:

```
\    /
 \  /
  \/
```

Considering the edges of the matrix as boundaries, this divides the grid into three triangles, so you should return `3`.

## Example

### Input
```
\    /
 \  /
  \/
```
### Output
```
3
```

## Explanation

Given a grid of forward slashes, backslashes, and blanks, count the number of distinct regions the slashes carve the space into (treating the matrix edges as boundaries).

## Company

Uber
