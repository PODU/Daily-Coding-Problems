// Greedy: sort intervals by end ascending; keep if start >= last kept end; count removals. O(n log n) time, O(1) extra space.
package main

import (
	"fmt"
	"math"
	"sort"
)

func eraseOverlapIntervals(intervals [][2]int) int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})
	removals := 0
	lastEnd := math.MinInt64
	for _, iv := range intervals {
		if iv[0] >= lastEnd {
			lastEnd = iv[1]
		} else {
			removals++
		}
	}
	return removals
}

func main() {
	intervals := [][2]int{{7, 9}, {2, 4}, {5, 8}}
	fmt.Println(eraseOverlapIntervals(intervals))
}
