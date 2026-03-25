// Day 1264: Largest rectangle of 1's in a binary matrix.
// Per-row histogram + largest-rectangle-in-histogram via monotonic stack. O(N*M) time, O(M) space.
package main

import "fmt"

func largestInHistogram(h []int) int {
	best := 0
	st := []int{}
	for i := 0; i <= len(h); i++ {
		cur := 0
		if i < len(h) {
			cur = h[i]
		}
		for len(st) > 0 && h[st[len(st)-1]] >= cur {
			height := h[st[len(st)-1]]
			st = st[:len(st)-1]
			left := -1
			if len(st) > 0 {
				left = st[len(st)-1]
			}
			if a := height * (i - left - 1); a > best {
				best = a
			}
		}
		st = append(st, i)
	}
	return best
}

func maximalRectangle(m [][]int) int {
	if len(m) == 0 {
		return 0
	}
	cols := len(m[0])
	h := make([]int, cols)
	best := 0
	for _, row := range m {
		for j := 0; j < cols; j++ {
			if row[j] == 1 {
				h[j]++
			} else {
				h[j] = 0
			}
		}
		if a := largestInHistogram(h); a > best {
			best = a
		}
	}
	return best
}

func main() {
	m := [][]int{{1, 0, 0, 0}, {1, 0, 1, 1}, {1, 0, 1, 1}, {0, 1, 0, 0}}
	fmt.Println(maximalRectangle(m))
}
