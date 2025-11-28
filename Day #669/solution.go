// Day 669: Misere Nim. First player wins iff either (some heap>1 and xor!=0) or
// (all heaps<=1 and xor==0). Time O(n), Space O(1).
package main

import "fmt"

func firstPlayerWins(heaps []int) bool {
	x, anyBig := 0, false
	for _, h := range heaps {
		x ^= h
		if h > 1 {
			anyBig = true
		}
	}
	if anyBig {
		return x != 0
	}
	return x == 0
}

func main() {
	if firstPlayerWins([]int{3, 4, 5}) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
