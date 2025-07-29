// Day 47: Max profit from one buy-then-sell. Track min price so far.
// Time: O(n), Space: O(1).
package main

import (
	"fmt"
	"math"
)

func maxProfit(prices []int) int {
	minPrice := math.MaxInt64
	best := 0
	for _, p := range prices {
		if p < minPrice {
			minPrice = p
		}
		if p-minPrice > best {
			best = p - minPrice
		}
	}
	return best
}

func main() {
	fmt.Println(maxProfit([]int{9, 11, 8, 5, 7, 10}))
}
