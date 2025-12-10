// Day 731: Max profit from a single buy-then-sell.
// Approach: Track running minimum price and best profit in one pass.
// Time: O(n), Space: O(1).
package main

import "fmt"

func maxProfit(prices []int) int {
	minPrice := int(^uint(0) >> 1) // max int
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
	fmt.Println(maxProfit([]int{9, 11, 8, 5, 7, 10})) // 5
}
