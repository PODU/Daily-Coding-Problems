# Day 1219

## Difficulty

Easy

## Problem Statement

What does the below code snippet print out? How can we fix the anonymous functions to behave as we'd expect?

```
functions = []
for i in range(10):
    functions.append(lambda : i)

for f in functions:
    print(f())
```

## Explanation

Explain the output of the Python closure snippet (all lambdas capture the same variable i, so all print 9) and fix the anonymous functions to capture each value of i as expected.

## Company

Dropbox
