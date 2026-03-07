// Day 1171: Validate an American-style crossword grid.
// Checks rotational symmetry, white-square connectivity (BFS), and that every
// maximal horizontal/vertical white run has length >= 3.
// Time O(N^2), Space O(N^2).  '#' = black, '.' = white.
package main

import "fmt"

func isValidCrossword(g []string) bool {
	n := len(g)
	if n == 0 {
		return false
	}
	for _, r := range g {
		if len(r) != n {
			return false
		}
	}

	// 1. Rotational symmetry.
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if (g[i][j] == '.') != (g[n-1-i][n-1-j] == '.') {
				return false
			}
		}
	}

	// 2 & 3. White runs (rows then columns) must be >= 3.
	runsOk := func(get func(a, b int) byte) bool {
		for a := 0; a < n; a++ {
			run := 0
			for b := 0; b <= n; b++ {
				if b < n && get(a, b) == '.' {
					run++
				} else {
					if run > 0 && run < 3 {
						return false
					}
					run = 0
				}
			}
		}
		return true
	}
	if !runsOk(func(i, j int) byte { return g[i][j] }) {
		return false
	}
	if !runsOk(func(j, i int) byte { return g[i][j] }) {
		return false
	}

	// 4. Connectivity.
	type pt struct{ x, y int }
	var whites []pt
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			if g[i][j] == '.' {
				whites = append(whites, pt{i, j})
			}
		}
	}
	if len(whites) == 0 {
		return true
	}
	vis := make([][]bool, n)
	for i := range vis {
		vis[i] = make([]bool, n)
	}
	start := whites[0]
	vis[start.x][start.y] = true
	q := []pt{start}
	seen := 1
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for len(q) > 0 {
		c := q[len(q)-1]
		q = q[:len(q)-1]
		for _, d := range dirs {
			nx, ny := c.x+d[0], c.y+d[1]
			if nx >= 0 && nx < n && ny >= 0 && ny < n && !vis[nx][ny] && g[nx][ny] == '.' {
				vis[nx][ny] = true
				seen++
				q = append(q, pt{nx, ny})
			}
		}
	}
	return seen == len(whites)
}

func main() {
	g1 := []string{".....", ".....", ".....", ".....", "....."}
	g2 := []string{".#...", ".....", ".....", ".....", "...#."}
	for _, g := range [][]string{g1, g2} {
		if isValidCrossword(g) {
			fmt.Println("true")
		} else {
			fmt.Println("false")
		}
	}
}
