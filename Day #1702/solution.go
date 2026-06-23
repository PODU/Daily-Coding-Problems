// Largest rectangle of 1s: per-row histogram + largest-rectangle-in-histogram via stack.
// Time O(N*M), Space O(M).
package main

import "fmt"

func largestInHist(h []int) int {
	best := 0
	var st []int
	for i := 0; i <= len(h); i++ {
		cur := 0
		if i < len(h) {
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

func main() {
	mat := [][]int{{1, 0, 0, 0}, {1, 0, 1, 1}, {1, 0, 1, 1}, {0, 1, 0, 0}}
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
		if r := largestInHist(h); r > best {
			best = r
		}
	}
	fmt.Println(best)
}
