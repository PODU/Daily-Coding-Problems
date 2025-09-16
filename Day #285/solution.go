// Day 285: Count buildings (east->west) with a sunset (west) view.
// Single backward pass tracking running max. Time O(N), Space O(1).
package main

import (
	"fmt"
	"math"
)

func sunsetViews(h []int) int {
	count := 0
	maxSoFar := math.MinInt
	for i := len(h) - 1; i >= 0; i-- {
		if h[i] > maxSoFar {
			count++
			maxSoFar = h[i]
		}
	}
	return count
}

func main() {
	fmt.Println(sunsetViews([]int{3, 7, 8, 3, 6, 1})) // 3
}
