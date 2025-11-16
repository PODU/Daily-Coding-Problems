// Day 607: Min total movement to seat M people contiguously in a row.
// Approach: target = median of (pos[i]-i); cost = sum |(pos[i]-i) - median|. Time O(n), Space O(M).
package main

import (
	"fmt"
	"sort"
)

func minCost(seats []int) int {
	var b []int
	idx := 0
	for i, s := range seats {
		if s == 1 {
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
	for _, v := range b {
		d := v - med
		if d < 0 {
			d = -d
		}
		cost += d
	}
	return cost
}

func main() {
	seats := []int{0, 1, 1, 0, 1, 0, 0, 0, 1}
	fmt.Println(minCost(seats)) // 5
}
