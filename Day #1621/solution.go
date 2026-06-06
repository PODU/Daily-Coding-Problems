// Day 1621: First N regular (5-smooth/Hamming) numbers.
// DP merge with 3 pointers for factors 2,3,5. Time O(N), Space O(N).
package main

import (
	"fmt"
	"strings"
)

func regularNumbers(n int) []int64 {
	h := make([]int64, n)
	h[0] = 1
	i2, i3, i5 := 0, 0, 0
	for i := 1; i < n; i++ {
		n2, n3, n5 := h[i2]*2, h[i3]*3, h[i5]*5
		m := n2
		if n3 < m {
			m = n3
		}
		if n5 < m {
			m = n5
		}
		h[i] = m
		if m == n2 {
			i2++
		}
		if m == n3 {
			i3++
		}
		if m == n5 {
			i5++
		}
	}
	return h
}

func main() {
	r := regularNumbers(10)
	parts := make([]string, len(r))
	for i, v := range r {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, " "))
}
