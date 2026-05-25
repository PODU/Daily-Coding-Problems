// Greedy interval stabbing: sort by right endpoint; for the smallest uncovered right,
// pick point = max left among intervals containing it, cover them all. Time O(n^2), Space O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func stabPoints(intervals [][2]int) []int {
	iv := make([][2]int, len(intervals))
	copy(iv, intervals)
	sort.Slice(iv, func(a, b int) bool { return iv[a][1] < iv[b][1] })
	n := len(iv)
	covered := make([]bool, n)
	var points []int
	for i := 0; i < n; i++ {
		if covered[i] {
			continue
		}
		r := iv[i][1]
		point := iv[i][0]
		for j := i; j < n; j++ {
			if !covered[j] && iv[j][0] <= r && iv[j][0] > point {
				point = iv[j][0]
			}
		}
		points = append(points, point)
		for j := i; j < n; j++ {
			if !covered[j] && iv[j][0] <= point && point <= iv[j][1] {
				covered[j] = true
			}
		}
	}
	return points
}

func main() {
	pts := stabPoints([][2]int{{0, 3}, {2, 6}, {3, 4}, {6, 9}})
	strs := make([]string, len(pts))
	for i, p := range pts {
		strs[i] = fmt.Sprintf("%d", p)
	}
	fmt.Println("{" + strings.Join(strs, ", ") + "}")
}
