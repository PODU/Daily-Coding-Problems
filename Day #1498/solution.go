// Day 1498: Conway's Game of Life on an infinite board using a set of live
// {row,col} cells; per step tally neighbor counts over live cells.
// Time O(L) per step (L = live cells), Space O(L).
package main

import (
	"fmt"
	"strings"
)

type cell struct{ r, c int }

type GameOfLife struct {
	live map[cell]bool
}

func NewGame(cells []cell) *GameOfLife {
	g := &GameOfLife{live: make(map[cell]bool)}
	for _, c := range cells {
		g.live[c] = true
	}
	return g
}

func (g *GameOfLife) step() {
	counts := make(map[cell]int)
	for c := range g.live {
		for dr := -1; dr <= 1; dr++ {
			for dc := -1; dc <= 1; dc++ {
				if dr != 0 || dc != 0 {
					counts[cell{c.r + dr, c.c + dc}]++
				}
			}
		}
	}
	next := make(map[cell]bool)
	for c, n := range counts {
		if n == 3 || (n == 2 && g.live[c]) {
			next[c] = true
		}
	}
	g.live = next
}

func (g *GameOfLife) render() string {
	if len(g.live) == 0 {
		return "(empty)"
	}
	first := true
	var minR, maxR, minC, maxC int
	for c := range g.live {
		if first {
			minR, maxR, minC, maxC = c.r, c.r, c.c, c.c
			first = false
		}
		if c.r < minR {
			minR = c.r
		}
		if c.r > maxR {
			maxR = c.r
		}
		if c.c < minC {
			minC = c.c
		}
		if c.c > maxC {
			maxC = c.c
		}
	}
	var lines []string
	for r := minR; r <= maxR; r++ {
		var sb strings.Builder
		for c := minC; c <= maxC; c++ {
			if g.live[cell{r, c}] {
				sb.WriteByte('*')
			} else {
				sb.WriteByte('.')
			}
		}
		lines = append(lines, sb.String())
	}
	return strings.Join(lines, "\n")
}

func main() {
	game := NewGame([]cell{{0, 1}, {1, 1}, {2, 1}})
	steps := 2
	fmt.Println("Step 0:")
	fmt.Println(game.render())
	for s := 1; s <= steps; s++ {
		game.step()
		fmt.Printf("Step %d:\n", s)
		fmt.Println(game.render())
	}
}
