// DP states cash/hold; fee charged once per buy-sell on sell. Time O(n), Space O(1).
package main

import "fmt"

func maxProfit(prices []int, fee int) int {
	if len(prices) == 0 {
		return 0
	}
	cash, hold := 0, -prices[0]
	for i := 1; i < len(prices); i++ {
		if hold+prices[i]-fee > cash {
			cash = hold + prices[i] - fee
		}
		if cash-prices[i] > hold {
			hold = cash - prices[i]
		}
	}
	return cash
}

func main() {
	prices := []int{1, 3, 2, 8, 4, 10}
	fee := 2
	fmt.Println(maxProfit(prices, fee))
}
