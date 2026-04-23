// Connect 4 on a 7x6 grid. drop() places in lowest empty cell; after each move
// check 4-in-a-row in all 4 directions from the placed cell. Time O(1) per move.
package main

import "fmt"

const COLS, ROWS = 7, 6

type Connect4 struct {
	grid [ROWS][COLS]byte
	turn byte
}

func NewConnect4() *Connect4 {
	g := &Connect4{turn: 'R'}
	for r := 0; r < ROWS; r++ {
		for c := 0; c < COLS; c++ {
			g.grid[r][c] = '.'
		}
	}
	return g
}

func (g *Connect4) drop(col int) byte {
	if col < 0 || col >= COLS || g.grid[ROWS-1][col] != '.' {
		return 'X'
	}
	r := 0
	for g.grid[r][col] != '.' {
		r++
	}
	g.grid[r][col] = g.turn
	who := g.turn
	if g.turn == 'R' {
		g.turn = 'B'
	} else {
		g.turn = 'R'
	}
	if g.wins(r, col, who) {
		return who
	}
	return ' '
}

func (g *Connect4) wins(r, c int, p byte) bool {
	dirs := [4][2]int{{0, 1}, {1, 0}, {1, 1}, {1, -1}}
	for _, d := range dirs {
		cnt := 1
		for s := -1; s <= 1; s += 2 {
			for k := 1; k < 4; k++ {
				nr, nc := r+d[0]*k*s, c+d[1]*k*s
				if nr < 0 || nr >= ROWS || nc < 0 || nc >= COLS || g.grid[nr][nc] != p {
					break
				}
				cnt++
			}
		}
		if cnt >= 4 {
			return true
		}
	}
	return false
}

func (g *Connect4) show() {
	for r := ROWS - 1; r >= 0; r-- {
		for c := 0; c < COLS; c++ {
			fmt.Printf("%c ", g.grid[r][c])
		}
		fmt.Println()
	}
}

func main() {
	game := NewConnect4()
	var winner byte = ' '
	for _, m := range []int{0, 1, 0, 1, 0, 1, 0} { // Red wins vertically in column 0
		res := game.drop(m)
		if res == 'R' || res == 'B' {
			winner = res
			break
		}
	}
	game.show()
	if winner != ' ' {
		if winner == 'R' {
			fmt.Println("Red wins!")
		} else {
			fmt.Println("Black wins!")
		}
	} else {
		fmt.Println("No winner yet.")
	}
}
