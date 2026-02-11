# Day 1054

## Difficulty

Medium

## Problem Statement

Implement the singleton pattern with a twist. First, instead of storing one instance, store two instances. And in every even call of `getInstance()`, return the first instance and in every odd call of `getInstance()`, return the second instance.

## Explanation

Build a variant of the singleton pattern that holds two instances, alternating which one is returned on even and odd calls to getInstance().

## Company

Microsoft
