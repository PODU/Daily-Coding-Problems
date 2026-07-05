// Day 1770: Flood fill via BFS, 4-directional. Replace connected same-colored region.
// Time: O(rows*cols), Space: O(rows*cols) for the queue in worst case.
package main

import (
	"fmt"
	"strings"
)

func floodFill(g [][]byte, sr, sc int, color byte) {
	R, C := len(g), len(g[0])
	target := g[sr][sc]
	if target == color {
		return
	}
	q := [][2]int{{sr, sc}}
	g[sr][sc] = color
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(q) > 0 {
		cur := q[0]
		q = q[1:]
		for _, d := range dirs {
			nr, nc := cur[0]+d[0], cur[1]+d[1]
			if nr >= 0 && nr < R && nc >= 0 && nc < C && g[nr][nc] == target {
				g[nr][nc] = color
				q = append(q, [2]int{nr, nc})
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
		for j, ch := range row {
			parts[j] = string(ch)
		}
		fmt.Println(strings.Join(parts, " "))
	}
}
