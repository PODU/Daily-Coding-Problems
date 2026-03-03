# Day 1145

## Difficulty

Medium

## Problem Statement

Implement a data structure which carries out the following operations without resizing the underlying array:

* `add(value)`: Add a value to the set of values.
* `check(value)`: Check whether a value is in the set.

The `check` method may return occasional false positives (in other words, incorrectly identifying an element as part of the set), but should always correctly identify a true element.

## Explanation

Implement a fixed-size set structure (a Bloom filter) supporting add and check, where check may yield false positives but never false negatives.

## Company

Triplebyte
