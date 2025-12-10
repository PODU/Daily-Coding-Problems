# Day 727

## Difficulty

Easy

## Problem Statement

Compute the running median of a sequence of numbers. That is, given a stream of numbers, print out the median of the list so far on each new element.

Recall that the median of an even-numbered list is the average of the two middle numbers.

## Example

### Input
```
[2, 1, 5, 7, 2, 0, 5]
```
### Output
```
2
1.5
2
3.5
2
2
2
```

## Explanation

Maintain a running median over a stream of numbers, printing the current median after each element (typically using two heaps).

## Company

Microsoft
