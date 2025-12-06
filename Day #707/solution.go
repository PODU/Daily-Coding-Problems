// Day 707: Min broadcast range. Sort towers; for each listener binary-search the
// nearest tower, answer is max of those min distances. Time O((N+M)logM).
package main

import (
	"fmt"
	"sort"
)

func minRange(listeners, towers []int) int {
	sort.Ints(towers)
	ans := 0
	for _, x := range listeners {
		i := sort.SearchInts(towers, x)
		best := int(^uint(0) >> 1)
		if i < len(towers) && towers[i]-x < best {
			best = towers[i] - x
		}
		if i > 0 && x-towers[i-1] < best {
			best = x - towers[i-1]
		}
		if best > ans {
			ans = best
		}
	}
	return ans
}

func main() {
	fmt.Println(minRange([]int{1, 5, 11, 20}, []int{4, 8, 15}))
}
