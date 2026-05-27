# Day 1571

## Difficulty

Medium

## Problem Statement

Given a dictionary of words and a string made up of those words (no spaces), return the original sentence in a list. If there is more than one possible reconstruction, return any of them. If there is no possible reconstruction, then return null.

For example, given the set of words 'quick', 'brown', 'the', 'fox', and the string "thequickbrownfox", you should return ['the', 'quick', 'brown', 'fox'].

Given the set of words 'bed', 'bath', 'bedbath', 'and', 'beyond', and the string "bedbathandbeyond", return either ['bed', 'bath', 'and', 'beyond] or ['bedbath', 'and', 'beyond'].

## Example

### Input
```
words = ['quick', 'brown', 'the', 'fox'], s = "thequickbrownfox"
```
### Output
```
['the', 'quick', 'brown', 'fox']
```

## Explanation

Given a dictionary of words and a spaceless string formed from them, reconstruct the original sentence as a list of words, returning any valid split or null if none exists.

## Company

Microsoft
