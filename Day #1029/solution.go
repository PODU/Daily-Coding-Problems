// Day 1029: Minimum coins for n cents with {1,5,10,25}. Greedy is optimal for
// this canonical US denomination set. Time O(#denominations), Space O(1).
package main

import "fmt"

func minCoins(n int) int {
	count := 0
	for _, c := range []int{25, 10, 5, 1} {
		count += n / c
		n %= c
	}
	return count
}

func main() {
	fmt.Println(minCoins(16))
}
