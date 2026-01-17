// Minimum points covering all closed intervals: greedy, sort by END ascending; open a new
// group when start>anchor-end, pick each group's MAX start as its point. Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func minPoints(intervals [][2]int) []int {
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})
	points := []int{}
	have := false
	anchorEnd, groupMaxStart := 0, 0
	for _, iv := range intervals {
		if !have || iv[0] > anchorEnd {
			if have {
				points = append(points, groupMaxStart)
			}
			have = true
			anchorEnd = iv[1]
			groupMaxStart = iv[0]
		} else if iv[0] > groupMaxStart {
			groupMaxStart = iv[0]
		}
	}
	if have {
		points = append(points, groupMaxStart)
	}
	return points
}

func main() {
	intervals := [][2]int{{0, 3}, {2, 6}, {3, 4}, {6, 9}}
	pts := minPoints(intervals)
	strs := make([]string, len(pts))
	for i, p := range pts {
		strs[i] = strconv.Itoa(p)
	}
	fmt.Println("{" + strings.Join(strs, ", ") + "}")
}
