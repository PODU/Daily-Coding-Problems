// Day 119: Min points to stab all intervals. Greedy: sort by start desc, pick start
// of each not-yet-stabbed interval (mirror of the classic sort-by-end greedy). O(n log n).
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

func minCover(iv [][2]int) []int {
	sort.Slice(iv, func(i, j int) bool { return iv[i][0] > iv[j][0] })
	pts := []int{}
	has := false
	last := 0
	for _, in := range iv {
		if !has || last > in[1] {
			last = in[0]
			pts = append(pts, in[0])
			has = true
		}
	}
	sort.Ints(pts)
	return pts
}

func main() {
	r := minCover([][2]int{{0, 3}, {2, 6}, {3, 4}, {6, 9}})
	parts := []string{}
	for _, v := range r {
		parts = append(parts, strconv.Itoa(v))
	}
	fmt.Println("{" + strings.Join(parts, ", ") + "}") // {3, 6}
}
