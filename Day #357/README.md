# Day 357

## Difficulty

Hard

## Problem Statement

You are given a binary tree in a peculiar string representation. Each node is written in the form (lr), where l corresponds to the left child and r corresponds to the right child.

If either l or r is null, it will be represented as a zero. Otherwise, it will be represented by a new (lr) pair.

Here are a few examples:

 * A root node with no children: (00)
 * A root node with two children: ((00)(00))
 * An unbalanced tree with three consecutive left children: ((((00)0)0)0)

Given this representation, determine the depth of the tree.

## Example

### Input
```
((((00)0)0)0)
```
### Output
```
4
```

## Explanation

Parse a binary tree given in a nested (lr) string representation (where 0 denotes a null child) and compute the tree's depth.

## Company

LinkedIn
