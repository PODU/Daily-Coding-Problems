# Day 938

## Difficulty

Hard

## Problem Statement

You are given a circular lock with three wheels, each of which display the numbers 0 through 9 in order. Each of these wheels rotate clockwise and counterclockwise.

In addition, the lock has a certain number of "dead ends", meaning that if you turn the wheels to one of these combinations, the lock becomes stuck in that state and cannot be opened.

Let us consider a "move" to be a rotation of a single wheel by one digit, in either direction. Given a lock initially set to 000, a target combination, and a list of dead ends, write a function that returns the minimum number of moves required to reach the target state, or None if this is impossible.

## Explanation

Find the minimum number of single-digit wheel rotations to move a three-wheel circular lock from 000 to a target combination, avoiding dead-end states, or return None if unreachable.

## Company

Citrix
