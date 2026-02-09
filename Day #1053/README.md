# Day 1053

## Difficulty

Hard

## Problem Statement

A rule looks like this:

`A NE B`

This means this means point `A` is located northeast of point `B`.

`A SW C`

means that point `A` is southwest of `C`.

Given a list of rules, check if the sum of the rules validate.

## Example

### Input
```
A N B
B NE C
C N A
```
### Output
```
does not validate, since A cannot be both north and south of C.
```

A second example:

### Input
```
A NW B
A N B
```
### Output
```
is considered valid.
```

## Explanation

Given directional rules relating pairs of points (north, south, east, west combinations), determine whether the whole set of rules is internally consistent.

## Company

Uber
