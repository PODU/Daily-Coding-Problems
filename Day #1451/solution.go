// Day 1451: Next lexicographic permutation in place (wraps to smallest).
// Find rightmost ascent, swap with next-larger suffix element, reverse suffix.
// Time O(n), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

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
	for l, r := i+1, n-1; l < r; l, r = l+1, r-1 {
		a[l], a[r] = a[r], a[l]
	}
}

func show(a []int) {
	nextPermutation(a)
	strs := make([]string, len(a))
	for i, v := range a {
		strs[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(strs, ",") + "]")
}

func main() {
	show([]int{1, 2, 3}) // [1,3,2]
	show([]int{1, 3, 2}) // [2,1,3]
	show([]int{3, 2, 1}) // [1,2,3]
}
