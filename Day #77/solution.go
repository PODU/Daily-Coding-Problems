// Day 77: Merge overlapping intervals. Sort by start, then sweep merging.
// Time O(n log n), Space O(n).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func mergeIntervals(iv [][2]int) [][2]int {
	sort.Slice(iv, func(i, j int) bool { return iv[i][0] < iv[j][0] })
	res := [][2]int{}
	for _, p := range iv {
		if len(res) > 0 && p[0] <= res[len(res)-1][1] {
			if p[1] > res[len(res)-1][1] {
				res[len(res)-1][1] = p[1]
			}
		} else {
			res = append(res, p)
		}
	}
	return res
}

func main() {
	in := [][2]int{{1, 3}, {5, 8}, {4, 10}, {20, 25}}
	res := mergeIntervals(in)
	parts := []string{}
	for _, p := range res {
		parts = append(parts, fmt.Sprintf("(%d, %d)", p[0], p[1]))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
	// [(1, 3), (4, 10), (20, 25)]
}
