// Day 128: Tower of Hanoi - print all moves.
// Classic recursion. O(2^n) moves (optimal), O(n) recursion depth.
package main

import "fmt"

func hanoi(n, from, via, to int) {
	if n == 0 {
		return
	}
	hanoi(n-1, from, to, via)
	fmt.Printf("Move %d to %d\n", from, to)
	hanoi(n-1, via, from, to)
}

func main() {
	hanoi(3, 1, 2, 3)
}
