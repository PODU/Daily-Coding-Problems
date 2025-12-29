// Max profit single buy-then-sell: track min price so far and max profit in one pass. O(n) time, O(1) space.
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
