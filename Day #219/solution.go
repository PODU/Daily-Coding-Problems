// Day 219: Connect 4 (7 cols x 6 rows).
// Approach: drop into lowest empty cell; after each move scan 4 directions from it for 4-in-a-row.
// Win check O(1) per move; board O(W*H).
package main

import "fmt"

const W, H = 7, 6

type Connect4 struct {
	grid   [H][W]byte
	height [W]int
}

func NewConnect4() *Connect4 {
	c := &Connect4{}
	for r := 0; r < H; r++ {
		for col := 0; col < W; col++ {
			c.grid[r][col] = '.'
		}
	}
	return c
}

func (c *Connect4) drop(col int, player byte) int {
	if col < 0 || col >= W || c.height[col] >= H {
		return -1
	}
	r := c.height[col]
	c.height[col]++
	c.grid[r][col] = player
	return r
}

func (c *Connect4) wins(r, col int, p byte) bool {
	dirs := [4][2]int{{0, 1}, {1, 0}, {1, 1}, {1, -1}}
	for _, d := range dirs {
		cnt := 1
		for _, s := range []int{-1, 1} {
			rr, cc := r+d[0]*s, col+d[1]*s
			for rr >= 0 && rr < H && cc >= 0 && cc < W && c.grid[rr][cc] == p {
				cnt++
				rr += d[0] * s
				cc += d[1] * s
			}
		}
		if cnt >= 4 {
			return true
		}
	}
	return false
}

func main() {
	g := NewConnect4()
	cols := []int{0, 1, 0, 1, 0, 1, 0}
	players := []byte{'R', 'B', 'R', 'B', 'R', 'B', 'R'}
	var winner byte
	for i := range cols {
		row := g.drop(cols[i], players[i])
		if g.wins(row, cols[i], players[i]) {
			winner = players[i]
			break
		}
	}
	fmt.Printf("Player %c wins!\n", winner) // R wins vertically in column 0
}
