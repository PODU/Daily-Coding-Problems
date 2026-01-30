# Day 991

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

Explain what the given Python closure code prints (due to late binding) and how to fix it to print the apparently intended values.

## Company

Google
