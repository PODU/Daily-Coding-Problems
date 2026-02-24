// Day 1121 - Max stock profit with a transaction fee, unlimited transactions
// State machine DP: best cash (not holding) and best hold. Time: O(n), Space: O(1).
package main

import "fmt"

func maxi(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func maxProfit(prices []int, fee int) int {
	cash, hold := 0, -prices[0]
	for i := 1; i < len(prices); i++ {
		cash = maxi(cash, hold+prices[i]-fee)
		hold = maxi(hold, cash-prices[i])
	}
	return cash
}

func main() {
	fmt.Println(maxProfit([]int{1, 3, 2, 8, 4, 10}, 2)) // 9
}
