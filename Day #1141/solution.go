// Day 1141: Min cost to pack people (remove gaps).
// Transform p_i -> p_i - i, answer = sum |q_i - median(q)|. Time O(n log n), Space O(m).
package main

import (
	"fmt"
	"sort"
)

func minCost(seats []int) int {
	q := []int{}
	idx := 0
	for i, v := range seats {
		if v == 1 {
			q = append(q, i-idx)
			idx++
		}
	}
	if len(q) == 0 {
		return 0
	}
	sort.Ints(q)
	med := q[len(q)/2]
	cost := 0
	for _, v := range q {
		if v > med {
			cost += v - med
		} else {
			cost += med - v
		}
	}
	return cost
}

func main() {
	fmt.Println(minCost([]int{0, 1, 1, 0, 1, 0, 0, 0, 1})) // 5
}
