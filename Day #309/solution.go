// Day 309: Min movement to pack people with no gaps. Map p_i - i; cost minimized
// at the median of those values. O(M log M).
package main

import (
	"fmt"
	"sort"
)

func minCost(seats []int) int {
	var b []int
	idx := 0
	for i, v := range seats {
		if v == 1 {
			b = append(b, i-idx)
			idx++
		}
	}
	if len(b) == 0 {
		return 0
	}
	sort.Ints(b)
	med := b[len(b)/2]
	cost := 0
	for _, x := range b {
		d := x - med
		if d < 0 {
			d = -d
		}
		cost += d
	}
	return cost
}

func main() {
	fmt.Println(minCost([]int{0, 1, 1, 0, 1, 0, 0, 0, 1})) // 5
}
