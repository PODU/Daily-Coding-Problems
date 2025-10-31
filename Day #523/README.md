# Day 523

## Difficulty

Easy

## Problem Statement

Given integers `M` and `N`, write a program that counts how many positive integer pairs `(a, b)` satisfy the following conditions:

- `a + b = M`
- `a XOR b = N`

## Explanation

Count positive integer pairs (a, b) where a + b = M and a XOR b = N, using the bitwise relationship that a + b = (a XOR b) + 2*(a AND b).

## Company

Jane Street
