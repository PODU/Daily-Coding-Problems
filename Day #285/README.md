# Day 285

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
3, since the top floors of the buildings with heights 8, 6, and 1 all have an unobstructed view to the west.
```

## Explanation

Count buildings (east to west) that can see the sunset to the west, i.e. are taller than every building west of them.

## Company

Mailchimp
