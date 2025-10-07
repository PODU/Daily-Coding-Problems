# Day 383

## Difficulty

Medium

## Problem Statement

Implement the function `embolden(s, lst)` which takes in a string `s` and list of substrings `lst`, and wraps all substrings in `s` with an HTML bold tag `<b>` and `</b>`.

If two bold tags overlap or are contiguous, they should be merged.

## Example

### Input
```
s = abcdefg, lst = ["bc", "ef"]
s = abcdefg, lst = ["bcd", "def"]
```
### Output
```
a<b>bc</b>d<b>ef</b>g
a<b>bcdef</b>g
```

## Explanation

Wrap every occurrence of the given substrings in bold tags, merging tags that overlap or are contiguous.

## Company

Gusto
