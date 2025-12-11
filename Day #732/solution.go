// Day 732: Minimum boats (each holds <=2 people, weight limit k).
// Approach: Sort; two pointers pair lightest with heaviest when they fit.
// Time: O(n log n), Space: O(1).
package main

import (
	"fmt"
	"sort"
)

func numBoats(w []int, k int) int {
	sort.Ints(w)
	i, j, boats := 0, len(w)-1, 0
	for i <= j {
		if w[i]+w[j] <= k {
			i++
		}
		j--
		boats++
	}
	return boats
}

func main() {
	fmt.Println(numBoats([]int{100, 200, 150, 80}, 200)) // 3
}
