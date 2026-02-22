# Day 1114

## Difficulty

Easy

## Problem Statement

On election day, a voting machine writes data in the form `(voter_id, candidate_id)` to a text file. Write a program that reads this file as a stream and returns the top `3` candidates at any given time. If you find a voter voting more than once, report this as fraud.

## Explanation

Process a stream of (voter_id, candidate_id) records to maintain the top 3 candidates in real time while flagging any voter who votes more than once as fraud.

## Company

Uber
