// Connect 4 engine with O(1)-per-move win detection (scan 4 directions from last disc).
// Time per move O(1), Space O(R*C).
package main

import "fmt"

const R, C = 6, 7

type Connect4 struct {
	g      [R][C]byte
	cur    byte
	over   bool
	winner byte
}

func NewGame() *Connect4 {
	c := &Connect4{cur: 'R', winner: '.'}
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			c.g[i][j] = '.'
		}
	}
	return c
}

func (c *Connect4) drop(col int) int {
	if col < 0 || col >= C || c.over {
		return -1
	}
	for r := R - 1; r >= 0; r-- {
		if c.g[r][col] == '.' {
			c.g[r][col] = c.cur
			return r
		}
	}
	return -1
}

func (c *Connect4) won(r, col int) bool {
	p := c.g[r][col]
	dirs := [4][2]int{{0, 1}, {1, 0}, {1, 1}, {1, -1}}
	for _, d := range dirs {
		cnt := 1
		for _, s := range []int{-1, 1} {
			nr, nc := r+d[0]*s, col+d[1]*s
			for nr >= 0 && nr < R && nc >= 0 && nc < C && c.g[nr][nc] == p {
				cnt++
				nr += d[0] * s
				nc += d[1] * s
			}
		}
		if cnt >= 4 {
			return true
		}
	}
	return false
}

func (c *Connect4) full() bool {
	for j := 0; j < C; j++ {
		if c.g[0][j] == '.' {
			return false
		}
	}
	return true
}

func (c *Connect4) play(col int) bool {
	r := c.drop(col)
	if r < 0 {
		return false
	}
	if c.won(r, col) {
		c.over = true
		c.winner = c.cur
	} else if c.full() {
		c.over = true
	} else if c.cur == 'R' {
		c.cur = 'B'
	} else {
		c.cur = 'R'
	}
	return true
}

func (c *Connect4) show() {
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			if j > 0 {
				fmt.Print(" ")
			}
			fmt.Printf("%c", c.g[i][j])
		}
		fmt.Println()
	}
}

func main() {
	game := NewGame()
	for _, m := range []int{0, 1, 0, 1, 0, 1, 0} { // R wins vertically in column 0
		game.play(m)
	}
	game.show()
	if game.winner != '.' {
		fmt.Printf("Player %c wins!\n", game.winner)
	} else {
		fmt.Println("Draw")
	}
}
