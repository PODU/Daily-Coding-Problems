// Day 1477: First N regular numbers (only prime factors 2, 3, 5).
// DP with three pointers merging *2, *3, *5 sequences. Time O(N), Space O(N).
package main

import "fmt"

func min3(a, b, c int64) int64 {
	m := a
	if b < m {
		m = b
	}
	if c < m {
		m = c
	}
	return m
}

func regularNumbers(n int) []int64 {
	if n <= 0 {
		return []int64{}
	}
	h := make([]int64, n)
	h[0] = 1
	i2, i3, i5 := 0, 0, 0
	for k := 1; k < n; k++ {
		nxt := min3(h[i2]*2, h[i3]*3, h[i5]*5)
		h[k] = nxt
		if nxt == h[i2]*2 {
			i2++
		}
		if nxt == h[i3]*3 {
			i3++
		}
		if nxt == h[i5]*5 {
			i5++
		}
	}
	return h
}

func main() {
	fmt.Println(regularNumbers(10)) // [1 2 3 4 5 6 8 9 10 12]
}
