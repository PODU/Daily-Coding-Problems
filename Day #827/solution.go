// Day 827: Min broadcast range.
// Sort towers; for each listener binary-search nearest tower, take min distance;
// answer = max over listeners. Time O((N+M) log M), Space O(1).
package main

import (
	"fmt"
	"sort"
)

func minBroadcastRange(listeners, towers []int) int {
	sort.Ints(towers)
	ans := 0
	for _, l := range listeners {
		best := int(^uint(0) >> 1)
		i := sort.SearchInts(towers, l)
		if i < len(towers) && towers[i]-l < best {
			best = towers[i] - l
		}
		if i > 0 && l-towers[i-1] < best {
			best = l - towers[i-1]
		}
		if best > ans {
			ans = best
		}
	}
	return ans
}

func main() {
	fmt.Println(minBroadcastRange([]int{1, 5, 11, 20}, []int{4, 8, 15}))
}
