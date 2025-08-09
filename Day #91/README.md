# Day 91

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

Explain the output of a Python closure-in-a-loop snippet and fix the lambdas so each captures its own value of the loop variable.

## Company

Dropbox
