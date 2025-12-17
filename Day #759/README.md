# Day 759

## Difficulty

Medium

## Problem Statement

Given a string of digits, generate all possible valid IP address combinations.

IP addresses must follow the format A.B.C.D, where A, B, C, and D are numbers between `0` and `255`. Zero-prefixed numbers, such as `01` and `065`, are not allowed, except for `0` itself.

## Example

### Input
```
"2542540123"
```
### Output
```
['254.25.40.123', '254.254.0.123']
```

## Explanation

Generate every way to split a digit string into four valid IPv4 octets (0-255, no leading zeros).

## Company

Snapchat
