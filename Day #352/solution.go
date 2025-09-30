// Validate crossword: rotational symmetry, all white runs (H & V) length>=3, white cells connected. O(N^2).
package main

import "fmt"

func valid(g []string) bool {
	n := len(g)
	// 1. rotational symmetry
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if (g[i][j] == '#') != (g[n-1-i][n-1-j] == '#') {
				return false
			}
		}
	}
	// 2a. horizontal runs >= 3
	for i := 0; i < n; i++ {
		run := 0
		for j := 0; j <= n; j++ {
			if j < n && g[i][j] == '.' {
				run++
			} else {
				if run > 0 && run < 3 {
					return false
				}
				run = 0
			}
		}
	}
	// 2b. vertical runs >= 3
	for j := 0; j < n; j++ {
		run := 0
		for i := 0; i <= n; i++ {
			if i < n && g[i][j] == '.' {
				run++
			} else {
				if run > 0 && run < 3 {
					return false
				}
				run = 0
			}
		}
	}
	// 3. connectivity
	type cell struct{ i, j int }
	var whites []cell
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] == '.' {
				whites = append(whites, cell{i, j})
			}
		}
	}
	if len(whites) == 0 {
		return true
	}
	seen := make([][]bool, n)
	for i := range seen {
		seen[i] = make([]bool, n)
	}
	q := []cell{whites[0]}
	seen[whites[0].i][whites[0].j] = true
	cnt := 0
	dx := []int{1, -1, 0, 0}
	dy := []int{0, 0, 1, -1}
	for len(q) > 0 {
		c := q[len(q)-1]
		q = q[:len(q)-1]
		cnt++
		for d := 0; d < 4; d++ {
			ni, nj := c.i+dx[d], c.j+dy[d]
			if ni >= 0 && ni < n && nj >= 0 && nj < n && g[ni][nj] == '.' && !seen[ni][nj] {
				seen[ni][nj] = true
				q = append(q, cell{ni, nj})
			}
		}
	}
	return cnt == len(whites)
}

func main() {
	gridA := []string{".....", ".....", ".....", ".....", "....."}
	gridB := []string{"#....", ".....", ".....", ".....", "....."}
	fmt.Println(valid(gridA))
	fmt.Println(valid(gridB))
}
