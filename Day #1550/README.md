# Day 1550

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

## Example

### Input
```
functions = []
for i in range(10):
    functions.append(lambda : i)

for f in functions:
    print(f())
```
### Output
```
9 (printed ten times)
```

## Explanation

Explain the late-binding closure behavior that makes all lambdas print 9, and fix them (e.g. with a default argument) to capture each value of i.

## Company

Dropbox
