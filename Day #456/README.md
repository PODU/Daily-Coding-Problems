# Day 456

## Difficulty

Easy

## Problem Statement

You are given a list of data entries that represent entries and exits of groups of people into a building. An entry looks like this:

```
{"timestamp": 1526579928, count: 3, "type": "enter"}
```

This means 3 people entered the building. An exit looks like this:

```
{"timestamp": 1526580382, count: 2, "type": "exit"}
```

This means that 2 people exited the building. timestamp is in Unix time.

Find the busiest period in the building, that is, the time with the most people in the building. Return it as a pair of (start, end) timestamps. You can assume the building always starts off and ends up empty, i.e. with 0 people inside.

## Example

### Input
```
{"timestamp": 1526579928, count: 3, "type": "enter"}
{"timestamp": 1526580382, count: 2, "type": "exit"}
```
### Output
```
(start, end) timestamps of the busiest period
```

## Explanation

Process enter/exit events to find the (start, end) interval during which the building held the most people.

## Company

Amazon
