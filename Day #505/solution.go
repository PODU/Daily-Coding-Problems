// Day 505: Rotate array right by k in-place via three reversals.
// Time O(n), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func reverse(a []int, lo, hi int) {
	for lo < hi {
		a[lo], a[hi] = a[hi], a[lo]
		lo++
		hi--
	}
}

func rotateRight(a []int, k int) {
	n := len(a)
	if n == 0 {
		return
	}
	k %= n
	reverse(a, 0, n-1)
	reverse(a, 0, k-1)
	reverse(a, k, n-1)
}

func format(a []int) string {
	parts := make([]string, len(a))
	for i, v := range a {
		parts[i] = strconv.Itoa(v)
	}
	return "[" + strings.Join(parts, ", ") + "]"
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6, 7}
	rotateRight(a, 3)
	fmt.Println(format(a)) // [5, 6, 7, 1, 2, 3, 4]
}
