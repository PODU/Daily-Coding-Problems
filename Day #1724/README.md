# Day 1724

## Difficulty

Medium

## Problem Statement

You are given a huge list of airline ticket prices between different cities around the world on a given day. These are all direct flights. Each element in the list has the format (source_city, destination, price).

Consider a user who is willing to take up to k connections from their origin city A to their destination B. Find the cheapest fare possible for this journey and print the itinerary for that journey.

## Example

### Input
```
JFK to LAX with up to 3 connections, given flights:
[
    ('JFK', 'ATL', 150),
    ('ATL', 'SFO', 400),
    ('ORD', 'LAX', 200),
    ('LAX', 'DFW', 80),
    ('JFK', 'HKG', 800),
    ('ATL', 'ORD', 90),
    ('JFK', 'LAX', 500),
]
```
### Output
```
JFK -> ATL -> ORD -> LAX, costing $440
```

## Explanation

Given direct flights as (source, destination, price), find the cheapest itinerary from A to B using at most k connections, and print that itinerary.

## Company

Airbnb
