# Day 1152

## Difficulty

Medium

## Problem Statement

Given an absolute pathname that may have `.` or `..` as part of it, return the shortest standardized path.

## Example

### Input
```
/usr/bin/../bin/./scripts/../
```
### Output
```
/usr/bin/
```

## Explanation

Normalize a Unix-style absolute path by resolving the current-directory (.) and parent-directory (..) components into the shortest equivalent path.

## Company

Quora
