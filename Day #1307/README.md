# Day 1307

## Difficulty

Medium

## Problem Statement

Given a function that generates perfectly random numbers between 1 and k (inclusive), where k is an input, write a function that shuffles a deck of cards represented as an array using only swaps.

It should run in O(N) time.

Hint: Make sure each one of the 52! permutations of the deck is equally likely.

## Explanation

Using a uniform random number generator, write an O(N) in-place shuffle (Fisher-Yates) that makes every permutation of the deck equally likely.

## Company

Facebook
