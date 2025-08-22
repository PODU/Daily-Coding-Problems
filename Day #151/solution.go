// Day 151: Flood fill via BFS. Replace target pixel's connected same-colored
// region with new color. Time O(R*C), Space O(R*C).
package main

import (
	"fmt"
	"strings"
)

func floodFill(m [][]byte, r, c int, color byte) {
	R, C := len(m), len(m[0])
	target := m[r][c]
	if target == color {
		return
	}
	queue := [][2]int{{r, c}}
	m[r][c] = color
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		for _, d := range dirs {
			nx, ny := cur[0]+d[0], cur[1]+d[1]
			if nx >= 0 && nx < R && ny >= 0 && ny < C && m[nx][ny] == target {
				m[nx][ny] = color
				queue = append(queue, [2]int{nx, ny})
			}
		}
	}
}

func main() {
	m := [][]byte{
		{'B', 'B', 'W'},
		{'W', 'W', 'W'},
		{'W', 'W', 'W'},
		{'B', 'B', 'B'},
	}
	floodFill(m, 2, 2, 'G')
	for _, row := range m {
		parts := make([]string, len(row))
		for i, ch := range row {
			parts[i] = string(ch)
		}
		fmt.Println(strings.Join(parts, " "))
	}
}
