// Day 1476: Max profit from a single buy-then-sell.
// Track the minimum price so far and the best profit in one pass.
// Time O(N), Space O(1).
package main

import "fmt"

func maxProfit(prices []int) int {
	minPrice := int(^uint(0) >> 1)
	best := 0
	for _, p := range prices {
		if p < minPrice {
			minPrice = p
		} else if p-minPrice > best {
			best = p - minPrice
		}
	}
	return best
}

func main() {
	fmt.Println(maxProfit([]int{9, 11, 8, 5, 7, 10})) // 5
}
