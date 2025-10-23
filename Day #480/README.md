# Day 480

## Difficulty

Medium

## Problem Statement

Given a dictionary of words and a string made up of those words (no spaces), return the original sentence in a list. If there is more than one possible reconstruction, return any of them. If there is no possible reconstruction, then return null.

## Example

### Input
```
words = 'quick', 'brown', 'the', 'fox'
string = "thequickbrownfox"

words = 'bed', 'bath', 'bedbath', 'and', 'beyond'
string = "bedbathandbeyond"
```
### Output
```
['the', 'quick', 'brown', 'fox']

either ['bed', 'bath', 'and', 'beyond'] or ['bedbath', 'and', 'beyond']
```

## Explanation

Given a dictionary and a concatenated, space-free string, reconstruct a valid sentence (list of words); return any valid split, or null if none exists.

## Company

Microsoft
