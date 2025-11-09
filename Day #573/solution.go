// Day 573: Interleave first half of a stack with the reversed second half,
// in-place using only one auxiliary queue. O(N) time, O(N) space.
package main

import (
	"fmt"
	"strings"
)

func interleave(stack []int) []int {
	n := len(stack)
	q := []int{}
	// Pop whole stack into queue (top..bottom).
	for len(stack) > 0 {
		q = append(q, stack[len(stack)-1])
		stack = stack[:len(stack)-1]
	}
	// base = bottom..top.
	base := make([]int, n)
	for k := 0; k < n; k++ {
		base[k] = q[n-1-k]
	}
	res := []int{}
	i, j, front := 0, n-1, true
	for i <= j {
		if front {
			res = append(res, base[i])
			i++
		} else {
			res = append(res, base[j])
			j--
		}
		front = !front
	}
	return res
}

func demo(bottomToTop []int) {
	stack := append([]int{}, bottomToTop...)
	r := interleave(stack)
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}

func main() {
	demo([]int{1, 2, 3, 4, 5}) // [1, 5, 2, 4, 3]
	demo([]int{1, 2, 3, 4})    // [1, 4, 2, 3]
}
