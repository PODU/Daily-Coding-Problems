// Boats: <=2 people, weight limit k, minimize boats. Sort + two pointers.
// Pair lightest with heaviest if they fit, else heaviest alone. Time O(n log n), Space O(1).
package main

import (
	"fmt"
	"sort"
)

func numBoats(weights []int, k int) int {
	w := make([]int, len(weights))
	copy(w, weights)
	sort.Ints(w)
	l, h, boats := 0, len(w)-1, 0
	for l <= h {
		if w[l]+w[h] <= k {
			l++
		}
		h--
		boats++
	}
	return boats
}

func main() {
	fmt.Println(numBoats([]int{100, 200, 150, 80}, 200))
}
