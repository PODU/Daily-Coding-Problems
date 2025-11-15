// Day 601: Reconstruct a permutation of [0..N] matching +/- relations between neighbors.
// Approach: greedy low/high pointers (DI-match). Time O(n), Space O(n). Any consistent array is valid.
package main

import "fmt"

// signs[0] is the placeholder (None); signs[k] is '+' if a[k] > a[k-1], else '-'.
func reconstruct(signs []byte) []int {
	n := len(signs) // numbers 0..n-1
	low, high := 0, n-1
	res := make([]int, 0, n)
	for k := 1; k < n; k++ {
		if signs[k] == '+' {
			res = append(res, low)
			low++
		} else {
			res = append(res, high)
			high--
		}
	}
	res = append(res, low)
	return res
}

func main() {
	signs := []byte{' ', '+', '+', '-', '+'} // [None, +, +, -, +]
	fmt.Println(reconstruct(signs))
}
