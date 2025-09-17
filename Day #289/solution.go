// Misere Nim (3 heaps): first player wins iff
// (some heap>1 && xor!=0) || (all heaps<=1 && xor==0). Time: O(1), Space: O(1).
package main

import "fmt"

func firstPlayerWins(a, b, c int) bool {
	x := a ^ b ^ c
	anyBig := a > 1 || b > 1 || c > 1
	if anyBig {
		return x != 0
	}
	return x == 0
}

func main() {
	fmt.Println(firstPlayerWins(3, 4, 5))
}
