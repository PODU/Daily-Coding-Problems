// Day 864: Minimum rescue boats (<=2 people, total weight <= k).
// Approach: sort, greedily pair lightest with heaviest using two pointers.
// Time: O(n log n), Space: O(1).
package main

import (
	"fmt"
	"sort"
)

func numRescueBoats(weights []int, k int) int {
	sort.Ints(weights)
	i, j, boats := 0, len(weights)-1, 0
	for i <= j {
		if weights[i]+weights[j] <= k {
			i++
		}
		j--
		boats++
	}
	return boats
}

func main() {
	fmt.Println(numRescueBoats([]int{100, 200, 150, 80}, 200)) // 3
}
