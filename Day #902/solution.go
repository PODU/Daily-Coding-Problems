// Greedy: sort intervals by end; keep interval if start >= last kept end.
// Answer = total - kept. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"math"
	"sort"
)

func eraseOverlapIntervals(intervals [][2]int) int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][1] < intervals[j][1] })
	kept, lastEnd := 0, math.MinInt32
	for _, it := range intervals {
		if it[0] >= lastEnd {
			kept++
			lastEnd = it[1]
		}
	}
	return len(intervals) - kept
}

func main() {
	intervals := [][2]int{{7, 9}, {2, 4}, {5, 8}}
	fmt.Println(eraseOverlapIntervals(intervals))
}
