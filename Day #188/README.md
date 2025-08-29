# Day 188

## Difficulty

Medium

## Problem Statement

What will this code print out?

```python
def make_functions():
    flist = []

    for i in [1, 2, 3]:
        def print_i():
            print(i)
        flist.append(print_i)

    return flist

functions = make_functions()
for f in functions:
    f()
```

How can we make it print out what we apparently want?

## Explanation

Explain the output of a Python closure-in-a-loop snippet (late binding causes all functions to print 3) and how to fix it so each function prints its intended value.

## Company

Google
