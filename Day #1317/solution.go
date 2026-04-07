// Reconstruct a permutation of [0..N] consistent with up/down signs.
// Two-pointer (DI-match): '+' takes next low, '-' takes next high. Time O(N).
// Any consistent array is valid; README shows one such answer.
package main

import "fmt"

// signs[0] is the sentinel (None); signs[i] in {'+','-'} for i>=1.
func reconstruct(signs []byte) []int {
	n := len(signs)
	N := n - 1
	res := make([]int, 0, n)
	lo, hi := 0, N
	for i := 1; i < n; i++ {
		if signs[i] == '+' {
			res = append(res, lo)
			lo++
		} else {
			res = append(res, hi)
			hi--
		}
	}
	res = append(res, lo) // lo == hi for the final element
	return res
}

func consistent(s []byte, a []int) bool {
	for i := 1; i < len(s); i++ {
		if s[i] == '+' && !(a[i] > a[i-1]) {
			return false
		}
		if s[i] == '-' && !(a[i] < a[i-1]) {
			return false
		}
	}
	return true
}

func main() {
	signs := []byte{'#', '+', '+', '-', '+'} // '#' stands in for None
	a := reconstruct(signs)
	fmt.Printf("%v  consistent=%v\n", a, consistent(signs, a))
	// -> [0 1 4 2 3]  (README's [1, 2, 3, 0, 4] is another valid answer)
}
