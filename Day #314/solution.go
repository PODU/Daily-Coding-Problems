// Day 314: Min broadcast range = max over listeners of distance to nearest tower.
// Sort towers, binary search each listener. O((N+M) log M).
package main

import (
	"fmt"
	"sort"
)

func minRange(listeners, towers []int) int {
	t := append([]int(nil), towers...)
	sort.Ints(t)
	ans := 0
	for _, L := range listeners {
		idx := sort.SearchInts(t, L)
		best := 1 << 30
		if idx < len(t) && t[idx]-L < best {
			best = t[idx] - L
		}
		if idx > 0 && L-t[idx-1] < best {
			best = L - t[idx-1]
		}
		if best > ans {
			ans = best
		}
	}
	return ans
}

func main() {
	fmt.Println(minRange([]int{1, 5, 11, 20}, []int{4, 8, 15})) // 5
}
