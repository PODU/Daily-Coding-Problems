// Day 612: Min intervals to remove so the rest are non-overlapping (touching allowed).
// Approach: sort by end, greedily keep max non-overlapping; answer = total - kept. Time O(n log n).
package main

import (
	"fmt"
	"math"
	"sort"
)

func minRemovals(intervals [][2]int) int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][1] < intervals[j][1] })
	kept, end := 0, math.MinInt32
	for _, iv := range intervals {
		if iv[0] >= end {
			kept++
			end = iv[1]
		}
	}
	return len(intervals) - kept
}

func main() {
	intervals := [][2]int{{7, 9}, {2, 4}, {5, 8}}
	fmt.Println(minRemovals(intervals)) // 1
}
