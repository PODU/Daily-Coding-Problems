# Day 28

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
words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"], k = 16
```
### Output
```
["the  quick brown",
"fox  jumps  over",
"the   lazy   dog"]
```

## Explanation

Fully justify text so that each line is exactly length k, packing as many words as possible per line and distributing extra spaces evenly from the left.

## Company

Palantir
