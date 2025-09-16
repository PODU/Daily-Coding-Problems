// Day 283: First N regular (5-smooth) numbers via 3-pointer merge of 2x,3x,5x.
// Time O(N), Space O(N).
package main

import "fmt"

func regular(n int) []int64 {
	h := make([]int64, n)
	h[0] = 1
	i2, i3, i5 := 0, 0, 0
	for i := 1; i < n; i++ {
		n2, n3, n5 := h[i2]*2, h[i3]*3, h[i5]*5
		nxt := n2
		if n3 < nxt {
			nxt = n3
		}
		if n5 < nxt {
			nxt = n5
		}
		h[i] = nxt
		if nxt == n2 {
			i2++
		}
		if nxt == n3 {
			i3++
		}
		if nxt == n5 {
			i5++
		}
	}
	return h
}

func main() {
	fmt.Println(regular(10)) // [1 2 3 4 5 6 8 9 10 12]
}
