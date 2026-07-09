// k nearest points: stable sort by squared Euclidean distance, take first k.
// Time O(N log N), Space O(N).
package main

import (
	"fmt"
	"sort"
	"strings"
)

func main() {
	pts := [][2]int{{0, 0}, {5, 4}, {3, 1}}
	cx, cy, k := 1, 2, 2
	idx := make([]int, len(pts))
	for i := range idx {
		idx[i] = i
	}
	dist := func(p [2]int) int {
		return (p[0]-cx)*(p[0]-cx) + (p[1]-cy)*(p[1]-cy)
	}
	sort.SliceStable(idx, func(a, b int) bool {
		return dist(pts[idx[a]]) < dist(pts[idx[b]])
	})
	parts := make([]string, 0, k)
	for i := 0; i < k; i++ {
		parts = append(parts, fmt.Sprintf("(%d, %d)", pts[idx[i]][0], pts[idx[i]][1]))
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
