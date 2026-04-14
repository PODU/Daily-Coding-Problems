// Max profit with at most k transactions. If k>=n/2 -> unlimited (sum positive diffs).
// Else DP with buy/sell rolling arrays. Time O(n*k), Space O(k).
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
	for j := range buy {
		buy[j] = math.MinInt32
	}
	for _, p := range prices {
		for j := 1; j <= k; j++ {
			if sell[j-1]-p > buy[j] {
				buy[j] = sell[j-1] - p
			}
			if buy[j]+p > sell[j] {
				sell[j] = buy[j] + p
			}
		}
	}
	return sell[k]
}

func main() {
	fmt.Println(maxProfit(2, []int{5, 2, 4, 0, 1}))
}
