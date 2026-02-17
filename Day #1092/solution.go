// First N regular (Hamming) numbers via 3-pointer merge of {2,3,5} multiples. Time O(N), Space O(N).
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

func regular(n int) []int64 {
	h := make([]int64, n)
	h[0] = 1
	i2, i3, i5 := 0, 0, 0
	for i := 1; i < n; i++ {
		n2, n3, n5 := h[i2]*2, h[i3]*3, h[i5]*5
		m := min3(n2, n3, n5)
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
	fmt.Println(regular(10))
}
