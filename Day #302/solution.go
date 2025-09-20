// Day 302: Count regions divided by slashes. Split each cell into 4 triangles,
// union-find adjacent triangles. Time O(N*M*alpha), space O(N*M).
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

func union(a, b int) { parent[find(a)] = find(b) }

func countRegions(grid []string) int {
	n := len(grid)
	M := 0
	for _, r := range grid {
		if len(r) > M {
			M = len(r)
		}
	}
	g := make([]string, n)
	for i, r := range grid {
		for len(r) < M {
			r += " "
		}
		g[i] = r
	}
	parent = make([]int, n*M*4)
	for i := range parent {
		parent[i] = i
	}
	idx := func(i, j, t int) int { return (i*M+j)*4 + t } // 0=top,1=right,2=bottom,3=left
	for i := 0; i < n; i++ {
		for j := 0; j < M; j++ {
			c := g[i][j]
			if c == '/' {
				union(idx(i, j, 0), idx(i, j, 3))
				union(idx(i, j, 1), idx(i, j, 2))
			} else if c == '\\' {
				union(idx(i, j, 0), idx(i, j, 1))
				union(idx(i, j, 2), idx(i, j, 3))
			} else {
				union(idx(i, j, 0), idx(i, j, 1))
				union(idx(i, j, 1), idx(i, j, 2))
				union(idx(i, j, 2), idx(i, j, 3))
			}
			if j+1 < M {
				union(idx(i, j, 1), idx(i, j+1, 3))
			}
			if i+1 < n {
				union(idx(i, j, 2), idx(i+1, j, 0))
			}
		}
	}
	cnt := 0
	for x := 0; x < n*M*4; x++ {
		if find(x) == x {
			cnt++
		}
	}
	return cnt
}

func main() {
	grid := []string{"\\    /", " \\  /", "  \\/"}
	fmt.Println(countRegions(grid)) // 3
}
