// Day 1342: Pack the maximum number of dictionary words onto a letter grid (vertex-disjoint adjacency paths).
// Enumerate each word's placements (cell bitmasks) via DFS, then backtrack to select max disjoint set. Exponential worst case.
package main

import "fmt"

var (
	N, M       int
	grid       []string
	dr         = []int{-1, 1, 0, 0}
	dc         = []int{0, 0, -1, 1}
	placements [][]uint64
	best       int
)

func dfsWord(w string, idx, r, c int, mask uint64, out map[uint64]bool) {
	mask |= 1 << uint(r*M+c)
	if idx == len(w)-1 {
		out[mask] = true
		return
	}
	for k := 0; k < 4; k++ {
		nr, nc := r+dr[k], c+dc[k]
		if nr < 0 || nr >= N || nc < 0 || nc >= M {
			continue
		}
		if mask&(1<<uint(nr*M+nc)) != 0 {
			continue
		}
		if grid[nr][nc] != w[idx+1] {
			continue
		}
		dfsWord(w, idx+1, nr, nc, mask, out)
	}
}

func backtrack(i int, used uint64, count int) {
	if count+(len(placements)-i) <= best {
		return
	}
	if i == len(placements) {
		if count > best {
			best = count
		}
		return
	}
	backtrack(i+1, used, count)
	for _, m := range placements[i] {
		if m&used == 0 {
			backtrack(i+1, used|m, count+1)
		}
	}
}

func maxWords(dict []string, board []string) int {
	grid = board
	N = len(board)
	M = len(board[0])
	placements = nil
	for _, w := range dict {
		masks := map[uint64]bool{}
		for r := 0; r < N; r++ {
			for c := 0; c < M; c++ {
				if grid[r][c] == w[0] {
					dfsWord(w, 0, r, c, 0, masks)
				}
			}
		}
		if len(masks) > 0 {
			lst := make([]uint64, 0, len(masks))
			for m := range masks {
				lst = append(lst, m)
			}
			placements = append(placements, lst)
		}
	}
	best = 0
	backtrack(0, 0, 0)
	return best
}

func main() {
	dict := []string{"eat", "rain", "in", "rat"}
	board := []string{"ean", "tti", "ara"}
	fmt.Println(maxWords(dict, board))
}
