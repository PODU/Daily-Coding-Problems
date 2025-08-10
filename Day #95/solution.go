// Day 95: Next lexicographic permutation in place. Find rightmost ascent, swap
// with next-larger suffix element, reverse suffix. O(n) time, O(1) space.
package main

import "fmt"

func nextPermutation(a []int) {
	n := len(a)
	i := n - 2
	for i >= 0 && a[i] >= a[i+1] {
		i--
	}
	if i >= 0 {
		j := n - 1
		for a[j] <= a[i] {
			j--
		}
		a[i], a[j] = a[j], a[i]
	}
	for lo, hi := i+1, n-1; lo < hi; lo, hi = lo+1, hi-1 {
		a[lo], a[hi] = a[hi], a[lo]
	}
}

func main() {
	tests := [][]int{{1, 2, 3}, {1, 3, 2}, {3, 2, 1}}
	for _, a := range tests {
		nextPermutation(a)
		fmt.Println(a)
	}
	// [1 3 2] / [2 1 3] / [1 2 3]
}
