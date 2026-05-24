// Max profit single buy-then-sell: track running min price and best (price - min). Time O(n), Space O(1).
package main

import "fmt"

func maxProfit(prices []int) int {
	if len(prices) == 0 {
		return 0
	}
	minPrice := prices[0]
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
	prices := []int{9, 11, 8, 5, 7, 10}
	fmt.Println(maxProfit(prices))
}
