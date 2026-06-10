// Minimum stabbing points: greedy sort intervals by right endpoint; add right
// endpoint when current interval is unstabbed. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"math"
	"sort"
	"strconv"
	"strings"
)

func main() {
	intervals := [][2]int{{1, 4}, {4, 5}, {7, 9}, {9, 12}}
	sort.Slice(intervals, func(i, j int) bool { return intervals[i][1] < intervals[j][1] })
	points := []int{}
	last := math.MinInt64
	for _, iv := range intervals {
		if iv[0] > last {
			points = append(points, iv[1])
			last = iv[1]
		}
	}
	parts := make([]string, len(points))
	for i, p := range points {
		parts[i] = strconv.Itoa(p)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
