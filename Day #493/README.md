# Day 493

## Difficulty

Medium

## Problem Statement

You are given `n` numbers as well as `n` probabilities that sum up to 1. Write a function to generate one of the numbers with its corresponding probability.

You can generate random numbers between 0 and 1 uniformly.

## Example

### Input
```
numbers = [1, 2, 3, 4]
probabilities = [0.1, 0.5, 0.2, 0.2]
```
### Output
```
Returns 1 10% of the time, 2 50% of the time, and 3 and 4 20% of the time.
```

## Explanation

Sample a number according to a given discrete probability distribution using uniform random values, typically via cumulative probabilities.

## Company

Triplebyte
