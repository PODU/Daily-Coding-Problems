// Interval point stabbing: greedy, sort by right endpoint, pick endpoint when uncovered.
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"math"
	"sort"
	"strings"
)

func main() {
	iv := [][2]int{{1, 4}, {4, 5}, {7, 9}, {9, 12}}
	sort.Slice(iv, func(i, j int) bool { return iv[i][1] < iv[j][1] })
	var pts []int
	last := math.MinInt64
	for _, p := range iv {
		if p[0] > last {
			last = p[1]
			pts = append(pts, p[1])
		}
	}
	strs := make([]string, len(pts))
	for i, v := range pts {
		strs[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(strs, ", ") + "]")
}
