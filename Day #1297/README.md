# Day 1297

## Difficulty

Easy

## Problem Statement

Using a read7() method that returns 7 characters from a file, implement readN(n) which reads n characters.

For example, given a file with the content “Hello world”, three read7() returns “Hello w”, “orld” and then “”.

## Example

### Input
```
File content: "Hello world"
```
### Output
```
read7() returns "Hello w", "orld", ""
```

## Explanation

Build a readN(n) function on top of a given read7() primitive, buffering leftover characters so exactly n characters are returned per call.

## Company

Microsoft
