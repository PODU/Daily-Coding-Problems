# Day 391

## Difficulty

Hard

## Problem Statement

We have some historical clickstream data gathered from our site anonymously using cookies. The histories contain URLs that users have visited in chronological order.

Write a function that takes two users' browsing histories as input and returns the longest contiguous sequence of URLs that appear in both.

## Example

### Input
```
user1 = ['/home', '/register', '/login', '/user', '/one', '/two']
user2 = ['/home', '/red', '/login', '/user', '/one', '/pink']
```
### Output
```
['/login', '/user', '/one']
```

## Explanation

Find the longest contiguous run of URLs that appears in both users' browsing histories.

## Company

Facebook
