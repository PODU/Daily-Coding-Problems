// Day 673: K nearest points to a center. Sort by squared distance.
// Time O(n log n), Space O(n). No sqrt needed for comparison.
package main

import (
	"fmt"
	"sort"
	"strings"
)

func kNearest(pts [][2]int, c [2]int, k int) [][2]int {
	d2 := func(p [2]int) int {
		dx, dy := p[0]-c[0], p[1]-c[1]
		return dx*dx + dy*dy
	}
	idx := make([]int, len(pts))
	for i := range idx {
		idx[i] = i
	}
	sort.SliceStable(idx, func(a, b int) bool {
		da, db := d2(pts[idx[a]]), d2(pts[idx[b]])
		if da != db {
			return da < db
		}
		return idx[a] < idx[b]
	})
	if k > len(pts) {
		k = len(pts)
	}
	res := make([][2]int, k)
	for i := 0; i < k; i++ {
		res[i] = pts[idx[i]]
	}
	return res
}

func main() {
	pts := [][2]int{{0, 0}, {5, 4}, {3, 1}}
	r := kNearest(pts, [2]int{1, 2}, 2)
	parts := make([]string, len(r))
	for i, p := range r {
		parts[i] = fmt.Sprintf("(%d, %d)", p[0], p[1])
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]") // [(0, 0), (3, 1)]
}
