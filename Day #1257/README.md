# Day 1257

## Difficulty

Medium

## Problem Statement

You are the technical director of WSPT radio, serving listeners nationwide. For simplicity's sake we can consider each listener to live along a horizontal line stretching from `0` (west) to `1000` (east).

Given a list of `N` listeners, and a list of `M` radio towers, each placed at various locations along this line, determine what the minimum broadcast range would have to be in order for each listener's home to be covered.

## Example

### Input
```
listeners = [1, 5, 11, 20], towers = [4, 8, 15]
```
### Output
```
5
```

## Explanation

Given listener and tower positions on a line, find the smallest broadcast range such that every listener is within range of some tower.

## Company

Spotify
