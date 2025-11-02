# Day 537

## Difficulty

Easy

## Problem Statement

A Collatz sequence in mathematics can be defined as follows. Starting with any positive integer:

- if n is even, the next number in the sequence is n / 2
- if n is odd, the next number in the sequence is 3n + 1

It is conjectured that every such sequence eventually reaches the number `1`. Test this conjecture.

Bonus: What input `n <= 1000000` gives the longest sequence?

## Explanation

Verify the Collatz conjecture by iterating the sequence (halve if even, 3n+1 if odd) until reaching 1, and optionally find the longest such sequence for n up to 1,000,000.

## Company

Apple
