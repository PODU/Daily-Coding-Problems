// Day 770: Misere Nim forced-win check.
// If every heap == 1: first player wins iff count of heaps is even.
// Else: first player wins iff XOR of heaps != 0. O(N).
package main

import "fmt"

func firstPlayerWins(heaps []int) bool {
	xorSum, allOne := 0, true
	for _, h := range heaps {
		xorSum ^= h
		if h > 1 {
			allOne = false
		}
	}
	if allOne {
		return len(heaps)%2 == 0
	}
	return xorSum != 0
}

func main() {
	fmt.Println(firstPlayerWins([]int{3, 4, 5})) // true
}
