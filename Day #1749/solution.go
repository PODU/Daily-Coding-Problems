// Day 1749: Max profit with at most k transactions.
// DP with buy/sell states; if k>=n/2 it's unlimited (sum positive diffs).
// Time O(n*k) (or O(n) when unlimited), Space O(k).
package main

import (
	"fmt"
	"math"
)

func maxProfit(k int, prices []int) int {
	n := len(prices)
	if n < 2 || k == 0 {
		return 0
	}
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
		for j := 1; j <= k; j++ {
			if sell[j-1]-price > buy[j] {
				buy[j] = sell[j-1] - price
			}
			if buy[j]+price > sell[j] {
				sell[j] = buy[j] + price
			}
		}
	}
	return sell[k]
}

func main() {
	fmt.Println(maxProfit(2, []int{5, 2, 4, 0, 1})) // 3
}
