// Day 1322: Misere Nim (last stone loses) forced win for first player.
// Theorem: first player wins iff (some heap>1 and XOR!=0) OR (all heaps<=1 and XOR==0). O(n) time, O(1) space.
package main

import "fmt"

func firstPlayerWins(heaps []int) bool {
	x, maxHeap := 0, 0
	for _, h := range heaps {
		x ^= h
		if h > maxHeap {
			maxHeap = h
		}
	}
	if maxHeap <= 1 {
		return x == 0
	}
	return x != 0
}

func main() {
	fmt.Println(firstPlayerWins([]int{3, 4, 5})) // true
}
