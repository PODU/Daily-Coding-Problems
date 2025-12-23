// Next lexicographic permutation in place (classic next_permutation). O(n) time, O(1) extra space.
package main

import (
	"fmt"
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

func format(a []int) string {
	parts := make([]string, len(a))
	for i, v := range a {
		parts[i] = fmt.Sprintf("%d", v)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	for _, arr := range [][]int{{1, 2, 3}, {1, 3, 2}, {3, 2, 1}} {
		nextPermutation(arr)
		fmt.Println(format(arr))
	}
}
