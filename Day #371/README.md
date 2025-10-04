# Day 371

## Difficulty

Hard

## Problem Statement

You are given a series of arithmetic equations as a string, such as:

```
y = x + 1
5 = x + 3
10 = z + y + 2
```

The equations use addition only and are separated by newlines. Return a mapping of all variables to their values. If it's not possible, then return null.

## Example

### Input
```
y = x + 1
5 = x + 3
10 = z + y + 2
```
### Output
```
{
  x: 2,
  y: 3,
  z: 5
}
```

## Explanation

Solve a system of addition-only equations over variables and constants, returning the value of every variable, or null if the system is unsolvable.

## Company

Google
