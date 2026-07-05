# Day 1770

## Difficulty

Medium

## Problem Statement

Given a 2-D matrix representing an image, a location of a pixel in the screen and a color `C`, replace the color of the given pixel and all adjacent same colored pixels with `C`.

## Example

### Input
```
matrix:
B B W
W W W
W W W
B B B
location pixel: (2, 2)
color: 'G' for green
```
### Output
```
B B G
G G G
G G G
B B B
```

## Explanation

Implement a flood fill: starting from a given pixel, replace it and all connected same-colored pixels with a new color.

## Company

Unattributed
