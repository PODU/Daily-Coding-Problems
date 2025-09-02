// Day 200: Minimum points stabbing all intervals.
// Greedy: sort by right endpoint; pick the right end whenever current interval is unstabbed.
// Time: O(n log n), Space: O(1).
package main

import (
	"fmt"
	"math"
	"sort"
)

func stab(iv [][2]int) []int {
	sort.Slice(iv, func(i, j int) bool { return iv[i][1] < iv[j][1] })
	var pts []int
	last := math.MinInt64
	for _, p := range iv {
		if p[0] > last {
			last = p[1]
			pts = append(pts, last)
		}
	}
	return pts
}

func main() {
	fmt.Println(stab([][2]int{{1, 4}, {4, 5}, {7, 9}, {9, 12}})) // [4 9]
}
