// Boat rescue: min boats, <=2 people each, weight limit k.
// Greedy two-pointer: sort, pair lightest with heaviest if sum<=k. O(n log n) time, O(1) extra.
package main

import (
	"fmt"
	"sort"
)

func numRescueBoats(w []int, k int) int {
	sort.Ints(w)
	lo, hi, boats := 0, len(w)-1, 0
	for lo <= hi {
		if w[lo]+w[hi] <= k {
			lo++
		}
		hi--
		boats++
	}
	return boats
}

func main() {
	w := []int{100, 200, 150, 80}
	fmt.Println(numRescueBoats(w, 200)) // expected 3
}
