# Day 1407

## Difficulty

Medium

## Problem Statement

You are given an array representing the heights of neighboring buildings on a city street, from east to west. The city assessor would like you to write an algorithm that returns how many of these buildings have a view of the setting sun, in order to properly value the street.

Can you do this using just one forward pass through the array?

## Example

### Input
```
[3, 7, 8, 3, 6, 1]
```
### Output
```
3
```

## Explanation

Count how many buildings (given east to west) have an unobstructed view of the setting sun to the west, meaning no taller building stands to their west; in the example heights 8, 6, and 1 qualify.

## Company

Mailchimp
