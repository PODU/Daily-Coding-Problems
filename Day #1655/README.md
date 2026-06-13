# Day 1655

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

Explain the output of a Python closure over a loop variable (late binding) and how to fix it to capture each value.

## Company

Google
