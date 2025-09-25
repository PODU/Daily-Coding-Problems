# Day 330

## Difficulty

Hard

## Problem Statement

A Boolean formula can be said to be satisfiable if there is a way to assign truth values to each variable such that the entire formula evaluates to true.

For example, suppose we have the following formula, where the symbol denotes negation (NOT):

`(NOT c OR b) AND (b OR c) AND (NOT b OR c) AND (NOT c OR NOT a)`

One way to satisfy this formula would be to let `a = False`, `b = True`, and `c = True`.

This type of formula, with `AND` statements joining tuples containing exactly one `OR`, is known as `2-CNF`.

Given a `2-CNF` formula, find a way to assign truth values to satisfy it, or return `False` if this is impossible.

## Explanation

Given a 2-CNF Boolean formula (an AND of clauses each containing exactly two literals joined by OR), determine a satisfying truth assignment for all variables, or report that none exists.

## Company

Dropbox
