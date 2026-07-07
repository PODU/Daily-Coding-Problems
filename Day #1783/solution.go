// Count regions split by '/','\\',' ' via Union-Find: each cell = 4 triangles (T,R,B,L).
// Union within cell per char and across neighbors. Time O(R*C*alpha), Space O(R*C).
package main

import "fmt"

var parent []int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func union(a, b int) {
	parent[find(a)] = find(b)
}

func regions(grid []string) int {
	R, C := len(grid), len(grid[0])
	parent = make([]int, 4*R*C)
	for i := range parent {
		parent[i] = i
	}
	idx := func(r, c, t int) int { return 4*(r*C+c) + t }
	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			ch := grid[r][c]
			switch ch {
			case ' ':
				union(idx(r, c, 0), idx(r, c, 1))
				union(idx(r, c, 1), idx(r, c, 2))
				union(idx(r, c, 2), idx(r, c, 3))
			case '/':
				union(idx(r, c, 0), idx(r, c, 3))
				union(idx(r, c, 1), idx(r, c, 2))
			default: // '\\'
				union(idx(r, c, 0), idx(r, c, 1))
				union(idx(r, c, 2), idx(r, c, 3))
			}
			if c+1 < C {
				union(idx(r, c, 1), idx(r, c+1, 3))
			}
			if r+1 < R {
				union(idx(r, c, 2), idx(r+1, c, 0))
			}
		}
	}
	roots := make(map[int]bool)
	for i := 0; i < 4*R*C; i++ {
		roots[find(i)] = true
	}
	return len(roots)
}

func main() {
	grid := []string{
		"\\    /",
		" \\  / ",
		"  \\/  ",
	}
	fmt.Println(regions(grid)) // 3
}
