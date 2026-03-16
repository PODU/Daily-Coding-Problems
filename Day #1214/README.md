# Day 1214

## Difficulty

Medium

## Problem Statement

Given an unordered list of flights taken by someone, each represented as (origin, destination) pairs, and a starting airport, compute the person's itinerary. If no such itinerary exists, return null. If there are multiple possible itineraries, return the lexicographically smallest one. All flights must be used in the itinerary.

## Example

### Input
```
[('SFO', 'HKO'), ('YYZ', 'SFO'), ('YUL', 'YYZ'), ('HKO', 'ORD')], starting airport 'YUL'
```
### Output
```
['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
```

## Explanation

Reconstruct a full itinerary that uses every given flight starting from the given airport, returning the lexicographically smallest valid ordering, or null if no valid itinerary exists.

## Company

Facebook
