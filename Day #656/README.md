# Day 656

## Difficulty

Medium

## Problem Statement

Given a 2-D matrix representing an image, a location of a pixel in the screen and a color `C`, replace the color of the given pixel and all adjacent same colored pixels with `C`.

## Example

### Input
```
Matrix:
B B W
W W W
W W W
B B B

Location pixel: (2, 2), Color: 'G' (green)
```
### Output
```
B B G
G G G
G G G
B B B
```

## Explanation

Perform a flood fill: starting at the given pixel, replace it and all connected pixels of the same original color with the new color C.

## Company

Unattributed
