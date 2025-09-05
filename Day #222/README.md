# Day 222

## Difficulty

Medium

## Problem Statement

Given an absolute pathname that may have `.` or `..` as part of it, return the shortest standardized path.

## Example

### Input
```
"/usr/bin/../bin/./scripts/../"
```
### Output
```
"/usr/bin/"
```

## Explanation

Normalize an absolute file path by resolving `.` and `..` components into the shortest equivalent path.

## Company

Quora
