// Flood fill via BFS from start pixel; recolor only cells equal to original color.
// Guard against infinite loop when new color == original. Time O(rows*cols), space O(rows*cols).
package main

import (
	"fmt"
	"strings"
)

func floodFill(img [][]byte, sr, sc int, color byte) {
	rows, cols := len(img), len(img[0])
	orig := img[sr][sc]
	if orig == color {
		return
	}
	queue := [][2]int{{sr, sc}}
	img[sr][sc] = color
	dirs := [4][2]int{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}
	for len(queue) > 0 {
		cell := queue[0]
		queue = queue[1:]
		for _, d := range dirs {
			nr, nc := cell[0]+d[0], cell[1]+d[1]
			if nr >= 0 && nr < rows && nc >= 0 && nc < cols && img[nr][nc] == orig {
				img[nr][nc] = color
				queue = append(queue, [2]int{nr, nc})
			}
		}
	}
}

func main() {
	img := [][]byte{
		{'B', 'B', 'W'},
		{'W', 'W', 'W'},
		{'W', 'W', 'W'},
		{'B', 'B', 'B'},
	}
	floodFill(img, 2, 2, 'G')
	for _, row := range img {
		cells := make([]string, len(row))
		for j, ch := range row {
			cells[j] = string(ch)
		}
		fmt.Println(strings.Join(cells, " "))
	}
}
