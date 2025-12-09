# Day 719

## Difficulty

Easy

## Problem Statement

Spreadsheets often use this alphabetical encoding for its columns: "A", "B", "C", ..., "AA", "AB", ..., "ZZ", "AAA", "AAB", ....

Given a column number, return its alphabetical column id. For example, given `1`, return "A". Given `27`, return "AA".

## Example

### Input
```
1
27
```
### Output
```
"A"
"AA"
```

## Explanation

Convert a 1-based column number into its spreadsheet-style bijective base-26 alphabetical column id.

## Company

Dropbox
