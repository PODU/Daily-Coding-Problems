# Day 905

## Difficulty

Medium

## Problem Statement

Given a list of words, determine whether the words can be chained to form a circle. A word `X` can be placed in front of another word `Y` in a circle if the last character of `X` is same as the first character of `Y`.

## Example

### Input
```
['chair', 'height', 'racket', 'touch', 'tunic']
```
### Output
```
True (chair --> racket --> touch --> height --> tunic --> chair)
```

## Explanation

Determine whether all the words can be arranged into a circle where each word's last character matches the next word's first character (Eulerian circuit on a character graph).

## Company

Dropbox
