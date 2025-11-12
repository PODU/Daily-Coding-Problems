// Build histogram heights per row; largest rectangle in histogram via monotonic stack.
// Time O(N*M), Space O(M).
package main

import "fmt"

func largestInHistogram(h []int) int {
	st := []int{}
	best := 0
	n := len(h)
	for i := 0; i <= n; i++ {
		cur := 0
		if i < n {
			cur = h[i]
		}
		for len(st) > 0 && h[st[len(st)-1]] >= cur {
			height := h[st[len(st)-1]]
			st = st[:len(st)-1]
			left := -1
			if len(st) > 0 {
				left = st[len(st)-1]
			}
			if area := height * (i - left - 1); area > best {
				best = area
			}
		}
		st = append(st, i)
	}
	return best
}

func maximalRectangle(mat [][]int) int {
	if len(mat) == 0 {
		return 0
	}
	m := len(mat[0])
	h := make([]int, m)
	best := 0
	for _, row := range mat {
		for j := 0; j < m; j++ {
			if row[j] == 1 {
				h[j]++
			} else {
				h[j] = 0
			}
		}
		if v := largestInHistogram(h); v > best {
			best = v
		}
	}
	return best
}

func main() {
	mat := [][]int{
		{1, 0, 0, 0},
		{1, 0, 1, 1},
		{1, 0, 1, 1},
		{0, 1, 0, 0},
	}
	fmt.Println(maximalRectangle(mat))
}
