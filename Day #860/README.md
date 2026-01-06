# Day 860

## Difficulty

Hard

## Problem Statement

Implement regular expression matching with the following special characters:

 * `.` (period) which matches any single character
 * `*` (asterisk) which matches zero or more of the preceding element

That is, implement a function that takes in a string and a valid regular expression and returns whether or not the string matches the regular expression.

## Example

### Input
```
regex = "ra.", string = "ray"
regex = "ra.", string = "raymond"
regex = ".*at", string = "chat"
regex = ".*at", string = "chats"
```
### Output
```
true
false
true
false
```

## Explanation

Implement regular expression matching supporting '.' (any single character) and '*' (zero or more of the preceding element), returning whether the whole string matches the pattern.

## Company

Facebook
