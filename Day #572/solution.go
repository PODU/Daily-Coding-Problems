// Next greater permutation in-place (lexicographic). If none, wrap to smallest.
// Approach: find pivot, successor swap, reverse suffix. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

func nextPermutation(a []int) []int {
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
	for l, r := i+1, n-1; l < r; l, r = l+1, r-1 {
		a[l], a[r] = a[r], a[l]
	}
	return a
}

func run(a []int) {
	nextPermutation(a)
	parts := make([]string, len(a))
	for i, v := range a {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ",") + "]")
}

func main() {
	run([]int{1, 2, 3})
	run([]int{1, 3, 2})
	run([]int{3, 2, 1})
}
