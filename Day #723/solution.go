// Day 723: Smallest set of points stabbing all closed intervals.
// Approach: Sort by right endpoint; greedily pick the end of each uncovered interval.
// Time: O(n log n), Space: O(n). (Any minimum-size set is valid; {3,9} here.)
package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func minStabbingPoints(intervals [][2]int) []int {
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
	intervals := [][2]int{{0, 3}, {2, 6}, {3, 4}, {6, 9}}
	pts := minStabbingPoints(intervals)
	strs := make([]string, len(pts))
	for i, p := range pts {
		strs[i] = strconv.Itoa(p)
	}
	fmt.Println("{" + strings.Join(strs, ", ") + "}")
}
