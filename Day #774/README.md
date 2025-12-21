# Day 774

## Difficulty

Easy

## Problem Statement

Using a read7() method that returns 7 characters from a file, implement readN(n) which reads n characters.

## Example

### Input
```
file content "Hello world", three calls to read7()
```
### Output
```
"Hello w", "orld", ""
```

## Explanation

Build readN(n) on top of a fixed read7() primitive, buffering leftover characters between calls.

## Company

Microsoft
