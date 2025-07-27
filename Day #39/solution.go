// Game of Life on infinite board: track live cells in a set, count neighbors via a
// map over live cells + neighbors each step. O(live*9) per step. Print cropped board.
package main

import "fmt"

type Cell struct{ x, y int }

func step(live map[Cell]bool) map[Cell]bool {
	counts := make(map[Cell]int)
	for c := range live {
		for dx := -1; dx <= 1; dx++ {
			for dy := -1; dy <= 1; dy++ {
				if dx != 0 || dy != 0 {
					counts[Cell{c.x + dx, c.y + dy}]++
				}
			}
		}
	}
	next := make(map[Cell]bool)
	for cell, n := range counts {
		if n == 3 || (live[cell] && n == 2) {
			next[cell] = true
		}
	}
	return next
}

func printBoard(live map[Cell]bool) {
	first := true
	var minx, maxx, miny, maxy int
	for c := range live {
		if first {
			minx, maxx, miny, maxy = c.x, c.x, c.y, c.y
			first = false
			continue
		}
		if c.x < minx {
			minx = c.x
		}
		if c.x > maxx {
			maxx = c.x
		}
		if c.y < miny {
			miny = c.y
		}
		if c.y > maxy {
			maxy = c.y
		}
	}
	for y := miny; y <= maxy; y++ {
		row := ""
		for x := minx; x <= maxx; x++ {
			if live[Cell{x, y}] {
				row += "*"
			} else {
				row += "."
			}
		}
		fmt.Println(row)
	}
}

func main() {
	live := map[Cell]bool{{0, 1}: true, {1, 1}: true, {2, 1}: true}
	steps := 2
	for s := 0; s <= steps; s++ {
		fmt.Printf("Step %d:\n", s)
		printBoard(live)
		if s < steps {
			live = step(live)
		}
	}
}
