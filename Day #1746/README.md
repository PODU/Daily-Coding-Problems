# Day 1746

## Difficulty

Medium

## Problem Statement

You are given `n` numbers as well as `n` probabilities that sum up to 1. Write a function to generate one of the numbers with its corresponding probability.

You can generate random numbers between 0 and 1 uniformly.

## Example

### Input
```
numbers = [1, 2, 3, 4], probabilities = [0.1, 0.5, 0.2, 0.2]
```
### Output
```
1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time
```

## Explanation

Implement a weighted random sampler that returns each number with its given probability, using a uniform random generator.

## Company

Triplebyte
