// Day 191: Min intervals to remove = n - max non-overlapping set (touching allowed).
// Greedy by earliest end. Time O(n log n), Space O(1).
package main

import (
	"fmt"
	"math"
	"sort"
)

func minRemovals(iv [][2]int) int {
	sort.Slice(iv, func(i, j int) bool { return iv[i][1] < iv[j][1] })
	kept, end := 0, math.MinInt64
	for _, p := range iv {
		if p[0] >= end {
			kept++
			end = p[1]
		}
	}
	return len(iv) - kept
}

func main() {
	fmt.Println(minRemovals([][2]int{{7, 9}, {2, 4}, {5, 8}}))
}
