# Day 1651

## Difficulty

Hard

## Problem Statement

You are going on a road trip, and would like to create a suitable music playlist. The trip will require `N` songs, though you only have `M` songs downloaded, where `M < N`. A valid playlist should select each song at least once, and guarantee a buffer of `B` songs between repeats.

Given `N`, `M`, and `B`, determine the number of valid playlists.

## Explanation

Count the number of length-N playlists drawn from M songs where each song appears at least once and any repeat of a song is separated by at least B other songs.

## Company

Apple
