// Validate American crossword grid: each white cell in horiz & vert runs of len>=3, connected, 180-deg symmetric.
// Time O(N^2), Space O(N^2).
package main

import "fmt"

func isValidCrossword(g [][]int) bool {
	n := len(g)
	if n == 0 {
		return false
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] != g[n-1-i][n-1-j] {
				return false
			}
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] != 0 {
				continue
			}
			l, r := j, j
			for l-1 >= 0 && g[i][l-1] == 0 {
				l--
			}
			for r+1 < n && g[i][r+1] == 0 {
				r++
			}
			if r-l+1 < 3 {
				return false
			}
			t, b := i, i
			for t-1 >= 0 && g[t-1][j] == 0 {
				t--
			}
			for b+1 < n && g[b+1][j] == 0 {
				b++
			}
			if b-t+1 < 3 {
				return false
			}
		}
	}

	total := 0
	sr, sc := -1, -1
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] == 0 {
				total++
				if sr < 0 {
					sr, sc = i, j
				}
			}
		}
	}
	if total == 0 {
		return false
	}

	vis := make([][]bool, n)
	for i := range vis {
		vis[i] = make([]bool, n)
	}
	queue := [][2]int{{sr, sc}}
	vis[sr][sc] = true
	seen := 0
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		seen++
		for _, d := range dirs {
			nr, nc := cur[0]+d[0], cur[1]+d[1]
			if nr >= 0 && nr < n && nc >= 0 && nc < n && !vis[nr][nc] && g[nr][nc] == 0 {
				vis[nr][nc] = true
				queue = append(queue, [2]int{nr, nc})
			}
		}
	}
	return seen == total
}

func main() {
	valid := make([][]int, 5)
	for i := range valid {
		valid[i] = make([]int, 5)
	}
	if isValidCrossword(valid) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}

	invalid := make([][]int, 5)
	for i := range invalid {
		invalid[i] = make([]int, 5)
	}
	invalid[0][0] = 1
	if isValidCrossword(invalid) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
