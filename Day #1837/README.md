# Day 1837

## Difficulty

Hard

## Problem Statement

The stable marriage problem is defined as follows:

Suppose you have `N` men and `N` women, and each person has ranked their prospective opposite-sex partners in order of preference.

For example, if `N = 3`, the input could be something like this:

```
guy_preferences = {
    'andrew': ['caroline', 'abigail', 'betty'],
    'bill': ['caroline', 'betty', 'abigail'],
    'chester': ['betty', 'caroline', 'abigail'],
}

gal_preferences = {
    'abigail': ['andrew', 'bill', 'chester'],
    'betty': ['bill', 'andrew', 'chester'],
    'caroline': ['bill', 'chester', 'andrew']
}
```

Write an algorithm that pairs the men and women together in such a way that no two people of opposite sex would both rather be with each other than with their current partners.

## Example

### Input
```
guy_preferences = {
    'andrew': ['caroline', 'abigail', 'betty'],
    'bill': ['caroline', 'betty', 'abigail'],
    'chester': ['betty', 'caroline', 'abigail'],
}

gal_preferences = {
    'abigail': ['andrew', 'bill', 'chester'],
    'betty': ['bill', 'andrew', 'chester'],
    'caroline': ['bill', 'chester', 'andrew']
}
```
### Output
```
A stable matching of men to women
```

## Explanation

Solve the stable marriage problem: pair N men and N women by preference so that no two would both prefer each other over their assigned partners.

## Company

Amazon
