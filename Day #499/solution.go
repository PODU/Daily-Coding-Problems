// Validate American-style crossword grid: every white cell in a horizontal AND vertical run
// of length >=3, single connected component of white cells, 180-degree rotational symmetry.
// Time: O(N^2), Space: O(N^2).
package main

import "fmt"

func isCrossword(g []string) bool {
	n := len(g)
	if n == 0 {
		return false
	}

	// Rule 4: rotational symmetry.
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] != g[n-1-i][n-1-j] {
				return false
			}
		}
	}

	// Rules 1 & 2: runs of length >= 3 in both directions.
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] != '.' {
				continue
			}
			l := j
			for l > 0 && g[i][l-1] == '.' {
				l--
			}
			r := j
			for r < n-1 && g[i][r+1] == '.' {
				r++
			}
			if r-l+1 < 3 {
				return false
			}
			t := i
			for t > 0 && g[t-1][j] == '.' {
				t--
			}
			b := i
			for b < n-1 && g[b+1][j] == '.' {
				b++
			}
			if b-t+1 < 3 {
				return false
			}
		}
	}

	// Rule 3: connectivity via BFS.
	total := 0
	startR, startC := -1, -1
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] == '.' {
				total++
				if startR < 0 {
					startR, startC = i, j
				}
			}
		}
	}
	if total == 0 {
		return true
	}
	seen := make([][]bool, n)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	type cell struct{ x, y int }
	queue := []cell{{startR, startC}}
	seen[startR][startC] = true
	cnt := 0
	dx := []int{0, 0, 1, -1}
	dy := []int{1, -1, 0, 0}
	for len(queue) > 0 {
		c := queue[0]
		queue = queue[1:]
		cnt++
		for d := 0; d < 4; d++ {
			nx, ny := c.x+dx[d], c.y+dy[d]
			if nx >= 0 && nx < n && ny >= 0 && ny < n && g[nx][ny] == '.' && !seen[nx][ny] {
				seen[nx][ny] = true
				queue = append(queue, cell{nx, ny})
			}
		}
	}
	return cnt == total
}

func main() {
	valid := []string{".....", ".....", ".....", ".....", "....."}
	invalid := []string{"..#..", ".....", "#...#", ".....", "..#.."}
	fmt.Println(isCrossword(valid))
	fmt.Println(isCrossword(invalid))
}
