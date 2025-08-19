# Day 143

## Difficulty

Medium

## Problem Statement

Given a pivot `x`, and a list `lst`, partition the list into three parts.

 * The first part contains all elements in `lst` that are less than `x`
 * The second part contains all elements in `lst` that are equal to `x`
 * The third part contains all elements in `lst` that are larger than `x`

Ordering within a part can be arbitrary.

## Example

### Input
```
x = 10 and lst = [9, 12, 3, 5, 14, 10, 10]
```
### Output
```
[9, 3, 5, 10, 10, 12, 14]
```

## Explanation

Dutch national flag partition: rearrange the list so all elements less than the pivot come first, then equal, then greater.

## Company

Amazon
