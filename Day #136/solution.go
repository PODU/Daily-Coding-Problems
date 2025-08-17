// Largest rectangle of 1's: per-row histogram + largest-rectangle-in-histogram via monotonic stack.
// Time O(N*M), Space O(M).
package main

import "fmt"

func maximalRectangle(mat [][]int) int {
	if len(mat) == 0 || len(mat[0]) == 0 {
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
		st := []int{}
		for j := 0; j <= m; j++ {
			cur := 0
			if j < m {
				cur = h[j]
			}
			for len(st) > 0 && h[st[len(st)-1]] >= cur {
				height := h[st[len(st)-1]]
				st = st[:len(st)-1]
				width := j
				if len(st) > 0 {
					width = j - st[len(st)-1] - 1
				}
				if height*width > best {
					best = height * width
				}
			}
			st = append(st, j)
		}
	}
	return best
}

func main() {
	mat := [][]int{{1, 0, 0, 0}, {1, 0, 1, 1}, {1, 0, 1, 1}, {0, 1, 0, 0}}
	fmt.Println(maximalRectangle(mat))
}
