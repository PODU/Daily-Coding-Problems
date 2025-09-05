# Day 217

## Difficulty

Hard

## Problem Statement

We say a number is sparse if there are no adjacent ones in its binary representation. For example, `21` (10101) is sparse, but `22` (10110) is not. For a given input `N`, find the smallest sparse number greater than or equal to `N`.

Do this in faster than `O(N log N)` time.

## Explanation

Find the smallest number >= N whose binary representation has no two adjacent set bits, faster than O(N log N).

## Company

Oracle
