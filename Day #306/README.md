# Day 306

## Difficulty

Medium

## Problem Statement

You are given a list of N numbers, in which each number is located at most k places away from its sorted position. For example, if k = 1, a given element at index 4 might end up at indices 3, 4, or 5.

Come up with an algorithm that sorts this list in O(N log k) time.

## Explanation

Sort a nearly-sorted list where each element is at most k positions from its final spot, achieving O(N log k) time (typically using a min-heap of size k).

## Company

Palantir
