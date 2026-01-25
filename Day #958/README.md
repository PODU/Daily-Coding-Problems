# Day 958

## Difficulty

Medium

## Problem Statement

Given an unordered list of flights taken by someone, each represented as (origin, destination) pairs, and a starting airport, compute the person's itinerary. If no such itinerary exists, return null. If there are multiple possible itineraries, return the lexicographically smallest one. All flights must be used in the itinerary.

## Example

### Input
```
flights = [('SFO', 'HKO'), ('YYZ', 'SFO'), ('YUL', 'YYZ'), ('HKO', 'ORD')], start = 'YUL'
flights = [('SFO', 'COM'), ('COM', 'YYZ')], start = 'COM'
flights = [('A', 'B'), ('A', 'C'), ('B', 'C'), ('C', 'A')], start = 'A'
```
### Output
```
['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
null
['A', 'B', 'C', 'A', 'C']
```

## Explanation

Reconstruct a complete itinerary that uses every flight exactly once starting from the given airport, returning the lexicographically smallest valid ordering, or null if none exists.

## Company

Facebook
