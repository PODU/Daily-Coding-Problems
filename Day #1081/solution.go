// Flood fill via BFS from the seed pixel. Time O(R*C), Space O(R*C).
package main

import "fmt"

func floodFill(img [][]byte, sr, sc int, c byte) {
	src := img[sr][sc]
	if src == c {
		return
	}
	R, C := len(img), len(img[0])
	q := [][2]int{{sr, sc}}
	img[sr][sc] = c
	dx := []int{0, 0, 1, -1}
	dy := []int{1, -1, 0, 0}
	for len(q) > 0 {
		p := q[0]
		q = q[1:]
		for d := 0; d < 4; d++ {
			nr, nc := p[0]+dx[d], p[1]+dy[d]
			if nr >= 0 && nr < R && nc >= 0 && nc < C && img[nr][nc] == src {
				img[nr][nc] = c
				q = append(q, [2]int{nr, nc})
			}
		}
	}
}

func main() {
	img := [][]byte{{'B', 'B', 'W'}, {'W', 'W', 'W'}, {'W', 'W', 'W'}, {'B', 'B', 'B'}}
	floodFill(img, 2, 2, 'G')
	for _, row := range img {
		for i, ch := range row {
			if i > 0 {
				fmt.Print(" ")
			}
			fmt.Printf("%c", ch)
		}
		fmt.Println()
	}
}
