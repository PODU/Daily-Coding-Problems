// Greedy on canonical US denominations {25,10,5,1}: take largest coin each step.
// Time O(D) where D = #denominations; Space O(1).
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
	fmt.Println(minCoins(16)) // 3
}
