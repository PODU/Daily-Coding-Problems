// Reconstruct a permutation of [0..N] from +/- signs. Two-pointer, O(N).
// '+' takes the current low, '-' takes the current high; produces a valid order.
package main

import "fmt"

func reconstruct(signs []byte) []int { // '+'/'-' constraints (leading None dropped)
	L := len(signs) + 1
	N := L - 1
	res := make([]int, L)
	low, high := 0, N
	for j := 0; j < len(signs); j++ {
		if signs[j] == '+' {
			res[j] = low
			low++
		} else {
			res[j] = high
			high--
		}
	}
	res[L-1] = low
	return res
}

func main() {
	// input [None, +, +, -, +] -> constraints (+, +, -, +)
	signs := []byte{'+', '+', '-', '+'}
	fmt.Println(reconstruct(signs)) // a valid reconstruction, e.g. [0 1 4 2 3]
}
