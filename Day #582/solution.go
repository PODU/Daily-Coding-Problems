// Greedy interval stabbing: sort by right endpoint, pick right end when uncovered. Time O(n log n).
package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func stab(intervals [][2]int) []int {
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][1] < intervals[j][1] })
	points := []int{}
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
	pts := stab(intervals)
	parts := make([]string, len(pts))
	for i, x := range pts {
		parts[i] = strconv.Itoa(x)
	}
	fmt.Printf("[%s]\n", strings.Join(parts, ", "))
}
