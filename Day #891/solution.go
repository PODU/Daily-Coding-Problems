// Region cutting by slashes: split each cell into 4 triangles, union-find.
// Time: O(N*M * alpha), Space: O(N*M).
package main

import "fmt"

var par []int

func find(x int) int {
	for par[x] != x {
		par[x] = par[par[x]]
		x = par[x]
	}
	return x
}

func uni(a, b int) { par[find(a)] = find(b) }

func regions(g []string) int {
	n, m := len(g), len(g[0])
	par = make([]int, n*m*4)
	for i := range par {
		par[i] = i
	}
	for r := 0; r < n; r++ {
		for c := 0; c < m; c++ {
			base := (r*m + c) * 4
			ch := g[r][c]
			if ch == '/' {
				uni(base+0, base+3)
				uni(base+1, base+2)
			} else if ch == '\\' {
				uni(base+0, base+1)
				uni(base+2, base+3)
			} else {
				uni(base+0, base+1)
				uni(base+1, base+2)
				uni(base+2, base+3)
			}
			if c+1 < m {
				uni(base+1, ((r*m+c+1)*4)+3)
			}
			if r+1 < n {
				uni(base+2, (((r+1)*m+c)*4)+0)
			}
		}
	}
	cnt := 0
	for i := range par {
		if find(i) == i {
			cnt++
		}
	}
	return cnt
}

func main() {
	g := []string{
		"\\    /",
		" \\  / ",
		"  \\/  ",
	}
	fmt.Println(regions(g))
}
