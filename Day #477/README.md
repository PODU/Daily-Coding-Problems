# Day 477

## Difficulty

Easy

## Problem Statement

What does the below code snippet print out? How can we fix the anonymous functions to behave as we'd expect?

```python
functions = []
for i in range(10):
    functions.append(lambda : i)

for f in functions:
    print(f())
```

## Explanation

Explain the output of the given Python lambda closure snippet and fix the anonymous functions so each captures its own value of i.

## Company

Dropbox
