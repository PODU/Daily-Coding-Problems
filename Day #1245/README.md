# Day 1245

## Difficulty

Medium

## Problem Statement

Write an algorithm to justify text. Given a sequence of words and an integer line length k, return a list of strings which represents each line, fully justified.

More specifically, you should have as many words as possible in each line. There should be at least one space between each word. Pad extra spaces when necessary so that each line has exactly length k. Spaces should be distributed as equally as possible, with the extra spaces, if any, distributed starting from the left.

If you can only fit one word on a line, then you should pad the right-hand side with spaces.

Each word is guaranteed not to be longer than k.

## Example

### Input
```
["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"] and k = 16
```
### Output
```
["the  quick brown", # 1 extra space on the left
"fox  jumps  over", # 2 extra spaces distributed evenly
"the   lazy   dog"] # 4 extra spaces distributed evenly
```

## Explanation

Given a list of words and a line width k, format the words into fully justified lines of exactly length k, fitting as many words per line as possible and distributing padding spaces evenly with leftover spaces favoring the left.

## Company

Palantir
