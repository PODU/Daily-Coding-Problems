# Day 894

## Difficulty

Easy

## Problem Statement

Design and implement a HitCounter class that keeps track of requests (or hits). It should support the following operations:

 * `record(timestamp)`: records a hit that happened at `timestamp`
 * `total()`: returns the total number of hits recorded
 * `range(lower, upper)`: returns the number of hits that occurred between timestamps `lower` and `upper` (inclusive)

Follow-up: What if our system has limited memory?

## Explanation

Design a HitCounter that records timestamped hits and supports querying the total number of hits and the number of hits within a timestamp range.

## Company

Riot Games
