# Day 531

## Difficulty

Easy

## Problem Statement

Using a read7() method that returns 7 characters from a file, implement readN(n) which reads n characters.

## Example

### Input
```
A file with the content "Hello world"
```
### Output
```
Three read7() returns "Hello w", "orld" and then "".
```

## Explanation

Build readN(n) on top of read7() by buffering leftover characters from each 7-character read so you can return exactly n characters per call.

## Company

Microsoft
