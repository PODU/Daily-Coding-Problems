// Day 455: Conway's Game of Life on an infinite board.
// Map set of live cells; tally neighbours each tick. Time O(live) per step.
package main

import "fmt"

type Cell struct{ r, c int }

func step(live map[Cell]bool) map[Cell]bool {
	cnt := map[Cell]int{}
	for cell := range live {
		for dr := -1; dr <= 1; dr++ {
			for dc := -1; dc <= 1; dc++ {
				if dr != 0 || dc != 0 {
					cnt[Cell{cell.r + dr, cell.c + dc}]++
				}
			}
		}
	}
	next := map[Cell]bool{}
	for cell, n := range cnt {
		if n == 3 || (n == 2 && live[cell]) {
			next[cell] = true
		}
	}
	return next
}

func printBoard(live map[Cell]bool) {
	if len(live) == 0 {
		fmt.Println("(empty)")
		return
	}
	first := true
	var r0, r1, c0, c1 int
	for cell := range live {
		if first {
			r0, r1, c0, c1 = cell.r, cell.r, cell.c, cell.c
			first = false
			continue
		}
		if cell.r < r0 {
			r0 = cell.r
		}
		if cell.r > r1 {
			r1 = cell.r
		}
		if cell.c < c0 {
			c0 = cell.c
		}
		if cell.c > c1 {
			c1 = cell.c
		}
	}
	for r := r0; r <= r1; r++ {
		line := ""
		for c := c0; c <= c1; c++ {
			if live[Cell{r, c}] {
				line += "*"
			} else {
				line += "."
			}
		}
		fmt.Println(line)
	}
}

func main() {
	live := map[Cell]bool{{1, 0}: true, {1, 1}: true, {1, 2}: true}
	steps := 2
	fmt.Println("Step 0:")
	printBoard(live)
	for s := 1; s <= steps; s++ {
		live = step(live)
		fmt.Printf("Step %d:\n", s)
		printBoard(live)
	}
}
