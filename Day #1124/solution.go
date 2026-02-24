// Day 1124 - Minimum points to stab all intervals
// Greedy: sort by right endpoint, place a point at the end of each not-yet-
// stabbed interval. Time: O(n log n), Space: O(n).
package main

import (
	"fmt"
	"math"
	"sort"
)

func stab(intervals [][2]int) []int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})
	var points []int
	last := math.MinInt64
	for _, iv := range intervals {
		if iv[0] > last {
			last = iv[1]
			points = append(points, iv[1])
		}
	}
	return points
}

func main() {
	intervals := [][2]int{{1, 4}, {4, 5}, {7, 9}, {9, 12}}
	fmt.Println(stab(intervals)) // [4 9]
}
