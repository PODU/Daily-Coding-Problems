# Day 22

## Difficulty

Medium

## Problem Statement

Given a dictionary of words and a string made up of those words (no spaces), return the original sentence in a list. If there is more than one possible reconstruction, return any of them. If there is no possible reconstruction, then return null.

For example, given the set of words 'quick', 'brown', 'the', 'fox', and the string "thequickbrownfox", you should return ['the', 'quick', 'brown', 'fox'].

Given the set of words 'bed', 'bath', 'bedbath', 'and', 'beyond', and the string "bedbathandbeyond", return either ['bed', 'bath', 'and', 'beyond] or ['bedbath', 'and', 'beyond'].

## Example

### Input
```
words = {'quick', 'brown', 'the', 'fox'}, string = "thequickbrownfox"
```
### Output
```
['the', 'quick', 'brown', 'fox']
```

## Explanation

Split a space-less string back into a sentence using a given dictionary of words; return null if no valid split exists.

## Company

Microsoft
