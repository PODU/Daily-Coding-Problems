// Min broadcast range: sort towers, binary-search nearest per listener, answer = max of mins.
// Time O((N+M) log M), Space O(1).
package main

import (
	"fmt"
	"sort"
)

func minRange(listeners, towers []int) int {
	sort.Ints(towers)
	ans := 0
	for _, l := range listeners {
		i := sort.SearchInts(towers, l)
		best := int(^uint(0) >> 1)
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
	fmt.Println(minRange([]int{1, 5, 11, 20}, []int{4, 8, 15}))
}
