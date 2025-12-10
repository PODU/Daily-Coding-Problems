# Day 730

## Difficulty

Medium

## Problem Statement

What will this code print out?

```
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

Explain the output of a Python closure-in-a-loop (it prints 3, 3, 3 due to late binding) and show how to capture the loop variable so it prints 1, 2, 3.

## Company

Google
