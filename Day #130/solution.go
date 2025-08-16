// Day 130: Max profit with at most k buy/sell transactions.
// DP with hold/cash per transaction. O(n*k) time, O(k) space (greedy when k large).
package main

import (
	"fmt"
	"math"
)

func maxProfit(k int, p []int) int {
	n := len(p)
	if n == 0 || k == 0 {
		return 0
	}
	if k >= n/2 {
		prof := 0
		for i := 1; i < n; i++ {
			if p[i] > p[i-1] {
				prof += p[i] - p[i-1]
			}
		}
		return prof
	}
	buy := make([]int, k+1)
	sell := make([]int, k+1)
	for j := range buy {
		buy[j] = math.MinInt32
	}
	for _, price := range p {
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
	prices := []int{5, 2, 4, 0, 1}
	fmt.Println(maxProfit(2, prices)) // 3
}
