# Day 729

## Difficulty

Medium

## Problem Statement

You have access to ranked lists of songs for various users. Each song is represented as an integer, and more preferred songs appear earlier in each list. For example, the list `[4, 1, 7]` indicates that a user likes song `4` the best, followed by songs `1` and `7`.

Given a set of these ranked lists, interleave them to create a playlist that satisfies everyone's priorities.

## Example

### Input
```
{[1, 7, 3], [2, 1, 6, 7, 9], [3, 9, 5]}
```
### Output
```
[2, 1, 6, 7, 3, 9, 5]
```

## Explanation

Given several users' ranked preference lists, produce a single ordering (topological sort) consistent with every user's relative priorities.

## Company

Spotify
