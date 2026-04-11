// Min coins for US denominations {1,5,10,25} via greedy (canonical system).
// Time O(#denominations), Space O(1).
package main

import "fmt"

func minCoins(n int) int {
	coins := []int{25, 10, 5, 1}
	count := 0
	for _, c := range coins {
		count += n / c
		n %= c
	}
	return count
}

func main() {
	fmt.Println(minCoins(16))
}
