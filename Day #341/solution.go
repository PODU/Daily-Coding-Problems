// Max non-overlapping dictionary words on a board.
// (1) DFS enumerate every placement (bitmask of cells) per word. (2) Backtrack over
// placements choosing pairwise-disjoint sets to maximize count.
package main

import "fmt"

var (
	R, C       int
	board      [][]byte
	placements [][2]int // {wordIndex, mask}
	bestCount  int
)

func dfs(w string, idx, r, c, used int, masks map[int]bool) {
	if board[r][c] != w[idx] {
		return
	}
	cell := r*C + c
	if used&(1<<cell) != 0 {
		return
	}
	used |= 1 << cell
	if idx == len(w)-1 {
		masks[used] = true
		return
	}
	dr := []int{-1, 1, 0, 0}
	dc := []int{0, 0, -1, 1}
	for k := 0; k < 4; k++ {
		nr, nc := r+dr[k], c+dc[k]
		if nr >= 0 && nr < R && nc >= 0 && nc < C {
			dfs(w, idx+1, nr, nc, used, masks)
		}
	}
}

func backtrack(start, occupied, usedWords, count int) {
	if count > bestCount {
		bestCount = count
	}
	for i := start; i < len(placements); i++ {
		w, m := placements[i][0], placements[i][1]
		if usedWords&(1<<w) != 0 {
			continue
		}
		if occupied&m != 0 {
			continue
		}
		backtrack(i+1, occupied|m, usedWords|(1<<w), count+1)
	}
}

func main() {
	dict := []string{"eat", "rain", "in", "rat"}
	board = [][]byte{{'e', 'a', 'n'}, {'t', 't', 'i'}, {'a', 'r', 'a'}}
	R, C = len(board), len(board[0])
	for wi, w := range dict {
		masks := map[int]bool{}
		for r := 0; r < R; r++ {
			for c := 0; c < C; c++ {
				dfs(w, 0, r, c, 0, masks)
			}
		}
		for m := range masks {
			placements = append(placements, [2]int{wi, m})
		}
	}
	bestCount = 0
	backtrack(0, 0, 0, 0)
	fmt.Println(bestCount)
}
