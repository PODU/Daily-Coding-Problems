// First N regular (5-smooth / Hamming) numbers via 3-pointer dynamic programming.
// Each number is min of next multiples by 2, 3, 5.
// Time: O(N), Space: O(N).
package main

import (
	"fmt"
	"strings"
)

func regularNumbers(n int) []int64 {
	h := make([]int64, n)
	h[0] = 1
	i2, i3, i5 := 0, 0, 0
	min3 := func(a, b, c int64) int64 {
		m := a
		if b < m {
			m = b
		}
		if c < m {
			m = c
		}
		return m
	}
	for i := 1; i < n; i++ {
		n2, n3, n5 := h[i2]*2, h[i3]*3, h[i5]*5
		nx := min3(n2, n3, n5)
		h[i] = nx
		if nx == n2 {
			i2++
		}
		if nx == n3 {
			i3++
		}
		if nx == n5 {
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
	fmt.Println(strings.Join(parts, " ")) // 1 2 3 4 5 6 8 9 10 12
}
