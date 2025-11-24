# Day 652

## Difficulty

Hard

## Problem Statement

You are going on a road trip, and would like to create a suitable music playlist. The trip will require `N` songs, though you only have `M` songs downloaded, where `M < N`. A valid playlist should select each song at least once, and guarantee a buffer of `B` songs between repeats.

Given `N`, `M`, and `B`, determine the number of valid playlists.

## Explanation

Count the number of playlists of length N drawn from M songs such that every song appears at least once and no song repeats within B songs of itself.

## Company

Apple
