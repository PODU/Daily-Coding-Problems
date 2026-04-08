# Day 1324

## Difficulty

Hard

## Problem Statement

You are given an array of length 24, where each element represents the number of new subscribers during the corresponding hour. Implement a data structure that efficiently supports the following:

 * update(hour: int, value: int): Increment the element at index hour by value.
 * query(start: int, end: int): Retrieve the number of subscribers that have signed up between start and end (inclusive).

You can assume that all values get cleared at the end of the day, and that you will not be asked for start and end values that wrap around midnight.

## Explanation

Design a data structure over a 24-element array that supports point updates and efficient inclusive range-sum queries.

## Company

Twitter
