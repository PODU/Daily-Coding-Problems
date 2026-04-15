// Max profit, unlimited transactions with a fixed fee per completed sale.
// DP states cash/hold tracked greedily. Time O(n), Space O(1).
package main

import (
	"fmt"
	"math"
)

func maxProfit(prices []int, fee int) int {
	cash := 0
	hold := math.MinInt64 / 4
	for _, p := range prices {
		if cash-p > hold {
			hold = cash - p
		}
		if hold+p-fee > cash {
			cash = hold + p - fee
		}
	}
	return cash
}

func main() {
	prices := []int{1, 3, 2, 8, 4, 10}
	fee := 2
	fmt.Println(maxProfit(prices, fee))
}
