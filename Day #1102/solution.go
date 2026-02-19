// Day 1102: Min total movement to seat people contiguously (order preserved).
// person i lands at start+i; minimize sum|pos[i]-(start+i)| => shift b[i]=pos[i]-i
// to its median. Time: O(N). Space: O(M).
package main

import (
	"fmt"
	"sort"
)

func minCost(seats []int) int {
	var b []int
	p := 0
	for i, s := range seats {
		if s == 1 {
			b = append(b, i-p)
			p++
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
