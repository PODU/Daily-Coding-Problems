// Single pass from the west end (array right), tracking the tallest seen so far;
// a building has a view iff it is taller than everything to its west.
// Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func countSunsetViews(h []int) int {
	count := 0
	maxW := math.MinInt64
	for i := len(h) - 1; i >= 0; i-- {
		if h[i] > maxW {
			count++
			maxW = h[i]
		}
	}
	return count
}

func main() {
	fmt.Println(countSunsetViews([]int{3, 7, 8, 3, 6, 1})) // 3
}
