// Count regions carved by slashes via 4-triangle split + Union-Find.
// Time O(n*m a(n*m)), Space O(n*m).
package main

import "fmt"

var parent []int
var count int

func find(x int) int {
	for parent[x] != x {
		parent[x] = parent[parent[x]]
		x = parent[x]
	}
	return x
}

func unite(a, b int) {
	ra, rb := find(a), find(b)
	if ra != rb {
		parent[ra] = rb
		count--
	}
}

func regions(grid []string) int {
	rows := len(grid)
	cols := 0
	if rows > 0 {
		cols = len(grid[0])
	}
	parent = make([]int, 4*rows*cols)
	for i := range parent {
		parent[i] = i
	}
	count = len(parent)
	idx := func(r, c, k int) int { return 4*(r*cols+c) + k }
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			ch := grid[r][c]
			t, ri, b, le := idx(r, c, 0), idx(r, c, 1), idx(r, c, 2), idx(r, c, 3)
			if ch == '/' {
				unite(t, le)
				unite(ri, b)
			} else if ch == '\\' {
				unite(t, ri)
				unite(le, b)
			} else {
				unite(t, ri)
				unite(ri, b)
				unite(b, le)
			}
			if c+1 < cols {
				unite(ri, idx(r, c+1, 3))
			}
			if r+1 < rows {
				unite(b, idx(r+1, c, 0))
			}
		}
	}
	return count
}

func main() {
	grid := []string{"\\    /", " \\  / ", "  \\/  "}
	fmt.Println(regions(grid))
}
