# Day 613

## Difficulty

Easy

## Problem Statement

Implement a `PrefixMapSum` class with the following methods:

 * insert(key: str, value: int): Set a given key's value in the map. If the key already exists, overwrite the value.
 * sum(prefix: str): Return the sum of all values of keys that begin with a given prefix.

## Example

### Input
```
mapsum.insert("columnar", 3)
mapsum.sum("col")

mapsum.insert("column", 2)
mapsum.sum("col")
```
### Output
```
mapsum.sum("col") == 3
mapsum.sum("col") == 5
```

## Explanation

Implement a map supporting key/value insertion and a query returning the sum of values for all keys sharing a given prefix.

## Company

Google
