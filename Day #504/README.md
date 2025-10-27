# Day 504

## Difficulty

Easy

## Problem Statement

You run an e-commerce website and want to record the last `N` `order` ids in a log. Implement a data structure to accomplish this, with the following API:

 * record(order_id): adds the order_id to the log
 * get_last(i): gets the ith last element from the log. i is guaranteed to be smaller than or equal to N.

You should be as efficient with time and space as possible.

## Explanation

Design a data structure that records order ids and retrieves the ith most recent one, using a fixed-size circular buffer for time and space efficiency.

## Company

Twitter
