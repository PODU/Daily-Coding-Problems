// Min broadcast range: sort towers, binary-search nearest tower per listener, take max.
// Time O((N+M) log M), Space O(1) extra.
package main

import (
	"fmt"
	"sort"
)

func main() {
	listeners := []int{1, 5, 11, 20}
	towers := []int{4, 8, 15}
	sort.Ints(towers)
	ans := 0
	for _, L := range listeners {
		i := sort.SearchInts(towers, L)
		best := int(^uint(0) >> 1)
		if i < len(towers) && towers[i]-L < best {
			best = towers[i] - L
		}
		if i > 0 && L-towers[i-1] < best {
			best = L - towers[i-1]
		}
		if best > ans {
			ans = best
		}
	}
	fmt.Println(ans)
}
