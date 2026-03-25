# Day 1262

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

Explain the output of this Python closure-in-a-loop code and fix it so each function prints its intended value.

## Company

Google
