// Day 1446: Minimum set of points hitting every closed interval.
// Greedy: sort by right endpoint; whenever the current interval is unhit, pick
// its right endpoint. Time O(n log n), Space O(1). (Any minimal set is valid.)
package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func minStabbingSet(intervals [][2]int) []int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})
	var points []int
	last := math.MinInt64
	for _, iv := range intervals {
		if iv[0] > last {
			points = append(points, iv[1])
			last = iv[1]
		}
	}
	return points
}

func main() {
	intervals := [][2]int{{0, 3}, {2, 6}, {3, 4}, {6, 9}}
	pts := minStabbingSet(intervals)
	strs := make([]string, len(pts))
	for i, p := range pts {
		strs[i] = strconv.Itoa(p)
	}
	fmt.Println("{" + strings.Join(strs, ", ") + "}") // e.g. {3, 9}; {3, 6} also valid
}
