// Flood fill via BFS from start pixel, recoloring 4-connected same-color region.
// Time: O(rows*cols), Space: O(rows*cols).
package main

import (
	"fmt"
	"strings"
)

func floodFill(g [][]byte, sr, sc int, color byte) {
	rows, cols := len(g), len(g[0])
	start := g[sr][sc]
	if start == color {
		return
	}
	type pt struct{ r, c int }
	q := []pt{{sr, sc}}
	g[sr][sc] = color
	dr := []int{-1, 1, 0, 0}
	dc := []int{0, 0, -1, 1}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for d := 0; d < 4; d++ {
			nr, nc := cur.r+dr[d], cur.c+dc[d]
			if nr >= 0 && nr < rows && nc >= 0 && nc < cols && g[nr][nc] == start {
				g[nr][nc] = color
				q = append(q, pt{nr, nc})
			}
		}
	}
}

func main() {
	g := [][]byte{
		{'B', 'B', 'W'},
		{'W', 'W', 'W'},
		{'W', 'W', 'W'},
		{'B', 'B', 'B'},
	}
	floodFill(g, 2, 2, 'G')
	for _, row := range g {
		parts := make([]string, len(row))
		for i, ch := range row {
			parts[i] = string(ch)
		}
		fmt.Println(strings.Join(parts, " "))
	}
}
