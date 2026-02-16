# Day 1081

## Difficulty

Medium

## Problem Statement

Given a 2-D matrix representing an image, a location of a pixel in the screen and a color `C`, replace the color of the given pixel and all adjacent same colored pixels with `C`.

## Example

### Input
```
location pixel of (2, 2), and 'G' for green:

B B W
W W W
W W W
B B B
```
### Output
```
B B G
G G G
G G G
B B B
```

## Explanation

Implement flood fill: starting from a given pixel, replace it and all connected pixels of the same color with a new color C.

## Company

Unattributed
