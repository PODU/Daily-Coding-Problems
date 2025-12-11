// Day 733: Connect 4 on a 7x6 grid.
// Approach: 2D board + per-column heights; after each drop scan 4 directions from the
// placed disc for 4-in-a-row. Time: O(1) per move, Space: O(rows*cols).
package main

import "fmt"

const cols, rows = 7, 6

type connect4 struct {
	board  [rows][cols]byte
	height [cols]int
}

func newGame() *connect4 {
	g := &connect4{}
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			g.board[r][c] = '.'
		}
	}
	return g
}

func (g *connect4) drop(col int, color byte) bool {
	r := g.height[col]
	g.height[col]++
	g.board[r][col] = color
	dirs := [4][2]int{{0, 1}, {1, 0}, {1, 1}, {1, -1}}
	for _, d := range dirs {
		cnt := 1
		for s := -1; s <= 1; s += 2 {
			for k := 1; k < 4; k++ {
				nr, nc := r+d[0]*k*s, col+d[1]*k*s
				if nr >= 0 && nr < rows && nc >= 0 && nc < cols && g.board[nr][nc] == color {
					cnt++
				} else {
					break
				}
			}
		}
		if cnt >= 4 {
			return true
		}
	}
	return false
}

func (g *connect4) show() {
	for r := rows - 1; r >= 0; r-- {
		for c := 0; c < cols; c++ {
			fmt.Printf("%c", g.board[r][c])
			if c+1 < cols {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func main() {
	g := newGame()
	moves := []int{0, 0, 1, 1, 2, 2, 3}
	color := byte('R')
	won := false
	for _, m := range moves {
		won = g.drop(m, color)
		if won {
			break
		}
		if color == 'R' {
			color = 'B'
		} else {
			color = 'R'
		}
	}
	g.show()
	if won {
		if color == 'R' {
			fmt.Println("Red wins!")
		} else {
			fmt.Println("Black wins!")
		}
	} else {
		fmt.Println("No winner yet")
	}
}
