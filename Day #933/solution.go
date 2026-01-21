// Day 933: Reconstruct a permutation of [0..N] consistent with +/- signs.
// Two-pointer: '+' takes the current low, '-' takes the current high. O(n) time, O(n) space.
// Many arrangements are valid; README shows [1,2,3,0,4], this prints another valid one.
package main

import "fmt"

// signs[0] is the sentinel for None; signs[i] in {'+','-'} for i>=1.
func reconstruct(signs []byte) []int {
	n := len(signs)
	lo, hi := 0, n-1
	res := make([]int, 0, n)
	for i := 1; i < n; i++ {
		if signs[i] == '+' {
			res = append(res, lo)
			lo++
		} else {
			res = append(res, hi)
			hi--
		}
	}
	res = append(res, lo) // lo == hi
	return res
}

func main() {
	signs := []byte{'?', '+', '+', '-', '+'} // [None, +, +, -, +]
	fmt.Println(reconstruct(signs))          // e.g. [0 1 4 2 3] (a valid reconstruction)
}
