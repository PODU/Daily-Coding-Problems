// Day 415: Max stock profit, unlimited transactions with a per-transaction fee.
// DP two states: cash (no stock) and hold (holding). Time O(N), Space O(1).
package main

import "fmt"

func maxProfit(prices []int, fee int) int {
	if len(prices) == 0 {
		return 0
	}
	cash, hold := 0, -prices[0]
	for i := 1; i < len(prices); i++ {
		if v := hold + prices[i] - fee; v > cash {
			cash = v
		}
		if v := cash - prices[i]; v > hold {
			hold = v
		}
	}
	return cash
}

func main() {
	fmt.Println(maxProfit([]int{1, 3, 2, 8, 4, 10}, 2)) // 9
}
