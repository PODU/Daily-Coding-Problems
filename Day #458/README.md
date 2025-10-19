# Day 458

## Difficulty

Hard

## Problem Statement

A rule looks like this:

A NE B

This means this means point A is located northeast of point B.

A SW C

means that point A is southwest of C.

Given a list of rules, check if the sum of the rules validate. For example:

```
A N B
B NE C
C N A
```

does not validate, since A cannot be both north and south of C.

```
A NW B
A N B
```

is considered valid.

## Example

### Input
```
A N B
B NE C
C N A
```
### Output
```
Invalid (A cannot be both north and south of C)
```

## Explanation

Given directional rules between points, determine whether they are consistent without contradiction.

## Company

Uber
