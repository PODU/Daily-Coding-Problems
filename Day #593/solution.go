// Day 593: Count buildings with a view of the setting sun (west).
// Array is east->west (index 0 = east). A building sees the sunset iff it is
// taller than every building further west (higher index). Single right-to-left pass.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"math"
)

func countSunsetViews(h []int) int {
	count := 0
	maxSeen := math.MinInt32
	for i := len(h) - 1; i >= 0; i-- {
		if h[i] > maxSeen {
			count++
			maxSeen = h[i]
		}
	}
	return count
}

func main() {
	fmt.Println(countSunsetViews([]int{3, 7, 8, 3, 6, 1})) // 3
}
