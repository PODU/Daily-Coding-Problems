// Day 260: Reconstruct a permutation of [0..N] consistent with +/- sign array.
// Grow a list: on '+' append max+1, on '-' append min-1; shift by -min into [0..N].
// O(n) time, O(n) space.
package main

import "fmt"

func reconstruct(signs []int) []int { // signs[0] is sentinel (None)
	res := []int{0}
	curMax, curMin := 0, 0
	for i := 1; i < len(signs); i++ {
		if signs[i] > 0 {
			curMax++
			res = append(res, curMax)
		} else {
			curMin--
			res = append(res, curMin)
		}
	}
	off := -curMin
	for i := range res {
		res[i] += off
	}
	return res
}

func main() {
	signs := []int{0, 1, 1, -1, 1} // [None, +, +, -, +]
	r := reconstruct(signs)
	fmt.Print("[")
	for i, v := range r {
		if i > 0 {
			fmt.Print(", ")
		}
		fmt.Print(v)
	}
	fmt.Println("]")
}
