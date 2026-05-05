# Day 1474

## Difficulty

Easy

## Problem Statement

Boggle is a game played on a `4 x 4` grid of letters. The goal is to find as many words as possible that can be formed by a sequence of adjacent letters in the grid, using each cell at most once. Given a game board and a dictionary of valid words, implement a Boggle solver.

## Explanation

Search the grid (DFS from each cell over adjacent letters, no cell reused) to find all dictionary words that can be formed.

## Company

Facebook
