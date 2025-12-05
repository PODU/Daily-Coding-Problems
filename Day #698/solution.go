// Day 698: Count regions a grid of '/'/'\\'/' ' is divided into.
// Approach: split each cell into 4 triangles (top,right,bottom,left) and union by
// the slash type plus across neighbors (Union-Find). Time O(R*C*a), Space O(R*C).
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

func uni(a, b int) { parent[find(a)] = find(b) }

func regions(grid []string) int {
	R := len(grid)
	C := 0
	for _, s := range grid {
		if len(s) > C {
			C = len(s)
		}
	}
	g := make([][]rune, R)
	for r := 0; r < R; r++ {
		g[r] = []rune(grid[r])
		for len(g[r]) < C {
			g[r] = append(g[r], ' ')
		}
	}
	parent = make([]int, R*C*4)
	for i := range parent {
		parent[i] = i
	}
	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			base := (r*C + c) * 4 // 0=T,1=R,2=B,3=L
			ch := g[r][c]
			switch ch {
			case '/':
				uni(base, base+3)
				uni(base+1, base+2)
			case '\\':
				uni(base, base+1)
				uni(base+2, base+3)
			default:
				uni(base, base+1)
				uni(base+1, base+2)
				uni(base+2, base+3)
			}
			if c+1 < C {
				uni(base+1, (r*C+c+1)*4+3)
			}
			if r+1 < R {
				uni(base+2, ((r+1)*C+c)*4)
			}
		}
	}
	cnt := 0
	for i := range parent {
		if find(i) == i {
			cnt++
		}
	}
	return cnt
}

func main() {
	grid := []string{"\\    /", " \\  /", "  \\/"}
	fmt.Println(regions(grid)) // 3
}
