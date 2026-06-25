// Max words packed: boggle DFS to enumerate placements + backtracking over words. Exponential worst case, small dict.
package main

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

var (
	n     int
	board [][]byte
	taken [][]bool
	dr    = []int{-1, 1, 0, 0}
	dc    = []int{0, 0, -1, 1}
	words []string
	used  = map[string]bool{}
	best  int
)

func key(path map[int]bool) string {
	cells := make([]int, 0, len(path))
	for c := range path {
		cells = append(cells, c)
	}
	sort.Ints(cells)
	parts := make([]string, len(cells))
	for i, c := range cells {
		parts[i] = strconv.Itoa(c)
	}
	return strings.Join(parts, ",")
}

func dfs(r, c int, w string, idx int, path map[int]bool, found map[string][]int) {
	if r < 0 || r >= n || c < 0 || c >= n {
		return
	}
	cell := r*n + c
	if taken[r][c] || path[cell] || board[r][c] != w[idx] {
		return
	}
	path[cell] = true
	if idx == len(w)-1 {
		k := key(path)
		if _, ok := found[k]; !ok {
			cells := make([]int, 0, len(path))
			for cc := range path {
				cells = append(cells, cc)
			}
			found[k] = cells
		}
	} else {
		for d := 0; d < 4; d++ {
			dfs(r+dr[d], c+dc[d], w, idx+1, path, found)
		}
	}
	delete(path, cell)
}

func findPlacements(w string) [][]int {
	found := map[string][]int{}
	for i := 0; i < n; i++ {
		for j := 0; j < n; j++ {
			dfs(i, j, w, 0, map[int]bool{}, found)
		}
	}
	res := make([][]int, 0, len(found))
	for _, v := range found {
		res = append(res, v)
	}
	return res
}

func search() {
	if len(used) > best {
		best = len(used)
	}
	for _, w := range words {
		if used[w] {
			continue
		}
		for _, placement := range findPlacements(w) {
			for _, cell := range placement {
				taken[cell/n][cell%n] = true
			}
			used[w] = true
			search()
			delete(used, w)
			for _, cell := range placement {
				taken[cell/n][cell%n] = false
			}
		}
	}
}

func main() {
	board = [][]byte{[]byte("ean"), []byte("tti"), []byte("ara")}
	n = 3
	taken = make([][]bool, n)
	for i := range taken {
		taken[i] = make([]bool, n)
	}
	words = []string{"eat", "rain", "in", "rat"}
	search()
	fmt.Println(best)
}
