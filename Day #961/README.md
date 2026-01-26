# Day 961

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

Normalize an absolute Unix-style path by resolving `.` (current directory) and `..` (parent directory) segments into the shortest equivalent path.

## Company

Quora
