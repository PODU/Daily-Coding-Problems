// Day 870: Largest rectangle of 1's in a binary matrix.
// Approach: build per-row histogram of consecutive 1's, apply largest-rectangle-in-histogram.
// Time: O(N*M), Space: O(M).
package main

import "fmt"

func largestInHist(h []int) int {
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
			width := i
			if len(st) > 0 {
				width = i - st[len(st)-1] - 1
			}
			if height*width > best {
				best = height * width
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
		if v := largestInHist(h); v > best {
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
	fmt.Println(maximalRectangle(mat)) // 4
}
