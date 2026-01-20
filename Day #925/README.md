# Day 925

## Difficulty

Easy

## Problem Statement

Write a function to flatten a nested dictionary. Namespace the keys with a period.

For example, given the following dictionary:

```
{
    "key": 3,
    "foo": {
        "a": 5,
        "bar": {
            "baz": 8
        }
    }
}
```

it should become:

```
{
    "key": 3,
    "foo.a": 5,
    "foo.bar.baz": 8
}
```

You can assume keys do not contain dots in them, i.e. no clobbering will occur.

## Example

### Input
```
{
    "key": 3,
    "foo": {
        "a": 5,
        "bar": {
            "baz": 8
        }
    }
}
```
### Output
```
{
    "key": 3,
    "foo.a": 5,
    "foo.bar.baz": 8
}
```

## Explanation

Flatten a nested dictionary into a single-level dictionary, joining nested keys with periods.

## Company

Stripe
