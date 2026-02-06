# Day 1028

## Difficulty

Medium

## Problem Statement

The number 6174 is known as Kaprekar's contant, after the mathematician who discovered an associated property: for all four-digit numbers with at least two distinct digits, repeatedly applying a simple procedure eventually results in this value. The procedure is as follows:

 * For a given input x, create two new numbers that consist of the digits in x in ascending and descending order.
 * Subtract the smaller number from the larger number.

Write a function that returns how many steps this will take for a given input N.

## Example

### Input
```
1234
```
### Output
```
3

 * 4321 - 1234 = 3087
 * 8730 - 0378 = 8352
 * 8532 - 2358 = 6174
```

## Explanation

For a four-digit number N, count how many Kaprekar steps (subtracting the ascending-digit arrangement from the descending one) it takes to reach 6174.

## Company

Salesforce
