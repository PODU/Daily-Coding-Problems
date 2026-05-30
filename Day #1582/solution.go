// Day 1582: Connect 4 on a 7x6 grid with win detection (H/V/diagonal).
// drop() places a disc; win() scans 4 directions from last move. Time: O(1) per move; Space: O(rows*cols).
package main

import (
	"fmt"
	"strings"
)

const COLS, ROWS = 7, 6

type Connect4 struct{ g [ROWS][COLS]byte }

func NewGame() *Connect4 {
	c := &Connect4{}
	for r := 0; r < ROWS; r++ {
		for col := 0; col < COLS; col++ {
			c.g[r][col] = '.'
		}
	}
	return c
}

func (c *Connect4) drop(col int, p byte) int {
	for r := ROWS - 1; r >= 0; r-- {
		if c.g[r][col] == '.' {
			c.g[r][col] = p
			return r
		}
	}
	return -1
}

func (c *Connect4) win(r, col int) bool {
	p := c.g[r][col]
	dirs := [4][2]int{{0, 1}, {1, 0}, {1, 1}, {1, -1}}
	for _, d := range dirs {
		cnt := 1
		for _, s := range []int{-1, 1} {
			rr, cc := r+s*d[0], col+s*d[1]
			for rr >= 0 && rr < ROWS && cc >= 0 && cc < COLS && c.g[rr][cc] == p {
				cnt++
				rr += s * d[0]
				cc += s * d[1]
			}
		}
		if cnt >= 4 {
			return true
		}
	}
	return false
}

func (c *Connect4) show() {
	for r := 0; r < ROWS; r++ {
		var sb []string
		for col := 0; col < COLS; col++ {
			sb = append(sb, string(c.g[r][col]))
		}
		fmt.Println(strings.Join(sb, " "))
	}
}

func main() {
	game := NewGame()
	type mv struct {
		col int
		p   byte
	}
	moves := []mv{{0, 'R'}, {1, 'B'}, {0, 'R'}, {1, 'B'}, {0, 'R'}, {1, 'B'}, {0, 'R'}}
	var winner byte
	for _, m := range moves {
		r := game.drop(m.col, m.p)
		if game.win(r, m.col) {
			winner = m.p
			break
		}
	}
	game.show()
	if winner != 0 {
		fmt.Printf("Winner: %c\n", winner) // R
	} else {
		fmt.Println("Winner: none")
	}
}
