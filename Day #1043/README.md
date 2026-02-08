# Day 1043

## Difficulty

Medium

## Problem Statement

Given an `iterator` with methods `next()` and `hasNext()`, create a wrapper iterator, `PeekableInterface`, which also implements `peek()`. `peek` shows the next element that would be returned on `next()`.

Here is the interface:

```
class PeekableInterface(object):
    def __init__(self, iterator):
        pass

    def peek(self):
        pass

    def next(self):
        pass

    def hasNext(self):
        pass
```

## Explanation

Wrap an iterator to add a peek() method that returns the next element without consuming it, while still supporting next() and hasNext().

## Company

Google
