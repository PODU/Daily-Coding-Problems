// Conway's Game of Life on an infinite board: store live cells in a set; each step
// tally neighbor counts only around live cells, then apply the 4 rules.
// Time: O(L) per step (L live cells); Space: O(L). Printing bounds to the live region.
package main

import "fmt"

type cell struct{ r, c int }

type GameOfLife struct {
	live map[cell]bool
}

func NewGame(cells []cell) *GameOfLife {
	g := &GameOfLife{live: make(map[cell]bool)}
	for _, x := range cells {
		g.live[x] = true
	}
	return g
}

func (g *GameOfLife) step() {
	counts := make(map[cell]int)
	for x := range g.live {
		for dr := -1; dr <= 1; dr++ {
			for dc := -1; dc <= 1; dc++ {
				if dr != 0 || dc != 0 {
					counts[cell{x.r + dr, x.c + dc}]++
				}
			}
		}
	}
	next := make(map[cell]bool)
	for x, cnt := range counts {
		if cnt == 3 || (cnt == 2 && g.live[x]) {
			next[x] = true
		}
	}
	g.live = next
}

func (g *GameOfLife) print(stepNo int) {
	fmt.Printf("Step %d:\n", stepNo)
	if len(g.live) == 0 {
		fmt.Println("(empty)\n")
		return
	}
	first := true
	var minR, maxR, minC, maxC int
	for x := range g.live {
		if first {
			minR, maxR, minC, maxC = x.r, x.r, x.c, x.c
			first = false
		}
		if x.r < minR {
			minR = x.r
		}
		if x.r > maxR {
			maxR = x.r
		}
		if x.c < minC {
			minC = x.c
		}
		if x.c > maxC {
			maxC = x.c
		}
	}
	for r := minR; r <= maxR; r++ {
		row := ""
		for c := minC; c <= maxC; c++ {
			if g.live[cell{r, c}] {
				row += "*"
			} else {
				row += "."
			}
		}
		fmt.Println(row)
	}
	fmt.Println()
}

func main() {
	g := NewGame([]cell{{0, 0}, {0, 1}, {0, 2}}) // horizontal blinker
	steps := 2
	g.print(0)
	for s := 1; s <= steps; s++ {
		g.step()
		g.print(s)
	}
}
