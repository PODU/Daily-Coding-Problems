# Day 193

## Difficulty

Hard

## Problem Statement

Given a array of numbers representing the stock prices of a company in chronological order, write a function that calculates the maximum profit you could have made from buying and selling that stock. You're also given a number `fee` that represents a transaction fee for each buy and sell transaction.

You must buy before you can sell the stock, but you can make as many transactions as you like.

## Example

### Input
```
[1, 3, 2, 8, 4, 10], fee = 2
```
### Output
```
9
```

## Explanation

Compute the maximum profit from unlimited buy/sell transactions over a list of stock prices, subtracting a fixed fee per transaction. In the example, buy at 1/sell at 8 and buy at 4/sell at 10 gives 7 + 6 = 13 minus 4 in fees = 9.

## Company

Affirm
