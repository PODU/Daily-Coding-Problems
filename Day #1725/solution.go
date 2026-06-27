// Min intervals to remove for non-overlapping (touching allowed).
// Greedy: sort by end, keep intervals whose start >= last kept end. O(n log n) time, O(1) extra space.
package main

import (
	"fmt"
	"math"
	"sort"
)

func minRemovals(intervals [][2]int) int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][1] < intervals[j][1] })
	kept, lastEnd := 0, math.MinInt32
	for _, iv := range intervals {
		if iv[0] >= lastEnd {
			kept++
			lastEnd = iv[1]
		}
	}
	return len(intervals) - kept
}

func main() {
	intervals := [][2]int{{7, 9}, {2, 4}, {5, 8}}
	fmt.Println(minRemovals(intervals))
}
