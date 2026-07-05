# Day 1768

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

A NW B
A N B

is considered valid.
```

## Explanation

Given directional rules between points (N/S/E/W combinations), determine whether the entire set of rules is consistent and contains no contradictions.

## Company

Uber
