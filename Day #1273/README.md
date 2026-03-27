# Day 1273

## Difficulty

Easy

## Problem Statement

Implement a PrefixMapSum class with the following methods:

 * insert(key: str, value: int): Set a given key's value in the map. If the key already exists, overwrite the value.
 * sum(prefix: str): Return the sum of all values of keys that begin with a given prefix.

For example, you should be able to run the following code:

```
mapsum.insert("columnar", 3)
assert mapsum.sum("col") == 3

mapsum.insert("column", 2)
assert mapsum.sum("col") == 5
```

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
3
5
```

## Explanation

Implement a map supporting insertion of key/value pairs and querying the sum of values for all keys sharing a given prefix.

## Company

Google
