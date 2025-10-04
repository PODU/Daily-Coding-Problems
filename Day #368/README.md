# Day 368

## Difficulty

Hard

## Problem Statement

Implement a key value store, where keys and values are integers, with the following methods:

 * update(key, vl): updates the value at key to val, or sets it if doesn't exist
 * get(key): returns the value with key, or None if no such value exists
 * max_key(val): returns the largest key with value val, or None if no key with that value exists

## Example

### Input
```
kv.update(1, 1)
kv.update(2, 1)
kv.max_key(1)
```
### Output
```
2
```

## Explanation

Build an integer key-value store supporting update, get, and max_key(val), where max_key returns the largest key currently mapped to a given value.

## Company

Google
