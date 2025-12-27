# Day 805

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

Convert a numeric column index into its spreadsheet-style alphabetical column label (bijective base-26).

## Company

Dropbox
