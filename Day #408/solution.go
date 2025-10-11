// Day 408: Max profit with at most k stock transactions.
// Approach: DP tracking best buy/sell state per transaction in one pass.
// Time: O(n*k), Space: O(k). Example k=2, [5,2,4,0,1] -> 3.
package main

import (
	"fmt"
	"math"
)

func maxProfit(k int, prices []int) int {
	n := len(prices)
	if n == 0 || k == 0 {
		return 0
	}
	// If k >= n/2, unlimited transactions are possible.
	if k >= n/2 {
		profit := 0
		for i := 1; i < n; i++ {
			if prices[i] > prices[i-1] {
				profit += prices[i] - prices[i-1]
			}
		}
		return profit
	}
	buy := make([]int, k+1)
	sell := make([]int, k+1)
	for i := range buy {
		buy[i] = math.MinInt32
	}
	for _, price := range prices {
		for t := 1; t <= k; t++ {
			if sell[t-1]-price > buy[t] {
				buy[t] = sell[t-1] - price
			}
			if buy[t]+price > sell[t] {
				sell[t] = buy[t] + price
			}
		}
	}
	return sell[k]
}

func main() {
	fmt.Println(maxProfit(2, []int{5, 2, 4, 0, 1})) // 3
}
