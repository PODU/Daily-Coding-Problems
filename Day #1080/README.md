# Day 1080

## Difficulty

Easy

## Problem Statement

You are given a starting state `start`, a list of transition probabilities for a Markov chain, and a number of steps `num_steps`. Run the Markov chain starting from `start` for `num_steps` and compute the number of times we visited each state.

## Example

### Input
```
starting state a, number of steps 5000, and the following transition probabilities:

[
  ('a', 'a', 0.9),
  ('a', 'b', 0.075),
  ('a', 'c', 0.025),
  ('b', 'a', 0.15),
  ('b', 'b', 0.8),
  ('b', 'c', 0.05),
  ('c', 'a', 0.25),
  ('c', 'b', 0.25),
  ('c', 'c', 0.5)
]
```
### Output
```
{ 'a': 3012, 'b': 1656, 'c': 332 }
```

## Explanation

Simulate a Markov chain from a starting state for a given number of steps using the provided transition probabilities, counting visits to each state.

## Company

Google
