// Conway's Game of Life on an infinite board using a set of live coordinates.
// Each step counts neighbours only around live cells.
// Time: O(L) per step, Space: O(L).
package main

import "fmt"

type Cell struct{ x, y int }

func step(live map[Cell]bool) map[Cell]bool {
	cnt := map[Cell]int{}
	for c := range live {
		for dx := -1; dx <= 1; dx++ {
			for dy := -1; dy <= 1; dy++ {
				if dx != 0 || dy != 0 {
					cnt[Cell{c.x + dx, c.y + dy}]++
				}
			}
		}
	}
	nb := map[Cell]bool{}
	for cell, c := range cnt {
		if c == 3 || (c == 2 && live[cell]) {
			nb[cell] = true
		}
	}
	return nb
}

func render(live map[Cell]bool) {
	if len(live) == 0 {
		fmt.Println("(empty)")
		return
	}
	first := true
	var minx, maxx, miny, maxy int
	for c := range live {
		if first {
			minx, maxx, miny, maxy = c.x, c.x, c.y, c.y
			first = false
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
	for x := minx; x <= maxx; x++ {
		row := ""
		for y := miny; y <= maxy; y++ {
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
	live := map[Cell]bool{{1, 0}: true, {1, 1}: true, {1, 2}: true} // blinker
	steps := 2
	for i := 0; i <= steps; i++ {
		fmt.Printf("Step %d:\n", i)
		render(live)
		fmt.Println()
		live = step(live)
	}
}
