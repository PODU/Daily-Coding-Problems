# Day 538

## Difficulty

Hard

## Problem Statement

Given a set of characters `C` and an integer `k`, a De Bruijn sequence is a cyclic sequence in which every possible `k`-length string of characters in `C` occurs exactly once.

Create an algorithm that finds a De Bruijn sequence.

## Example

### Input
```
C = {0, 1}, k = 3
```
### Output
```
00010111
```

## Explanation

Construct a De Bruijn sequence: a cyclic string over alphabet C in which every length-k string appears exactly once, typically via an Eulerian path.

## Company

LinkedIn
