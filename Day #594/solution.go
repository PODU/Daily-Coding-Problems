// Day 594: Max number of non-overlapping dictionary words packable on a letter board.
// Approach: for each word enumerate all adjacency placements (DFS) as cell bitmasks,
// then backtracking max set-packing to pick the most pairwise-disjoint placements.
// Time: O(words * board * 4^L) to enumerate + exponential packing on small boards.
package main

import "fmt"

var board [][]byte
var R, C int

func findWord(w string, idx, r, c int, mask uint64, out map[uint64]bool) {
	if r < 0 || c < 0 || r >= R || c >= C {
		return
	}
	bit := uint64(1) << uint(r*C+c)
	if mask&bit != 0 || board[r][c] != w[idx] {
		return
	}
	mask |= bit
	if idx == len(w)-1 {
		out[mask] = true
		return
	}
	findWord(w, idx+1, r+1, c, mask, out)
	findWord(w, idx+1, r-1, c, mask, out)
	findWord(w, idx+1, r, c+1, mask, out)
	findWord(w, idx+1, r, c-1, mask, out)
}

var best int

func pack(placements []uint64, i int, used uint64, cnt int) {
	if cnt > best {
		best = cnt
	}
	for j := i; j < len(placements); j++ {
		if placements[j]&used == 0 {
			pack(placements, j+1, used|placements[j], cnt+1)
		}
	}
}

func maxWords(dict []string) int {
	var placements []uint64
	for _, w := range dict {
		masks := map[uint64]bool{}
		for r := 0; r < R; r++ {
			for c := 0; c < C; c++ {
				findWord(w, 0, r, c, 0, masks)
			}
		}
		for m := range masks {
			placements = append(placements, m)
		}
	}
	best = 0
	pack(placements, 0, 0, 0)
	return best
}

func main() {
	board = [][]byte{{'e', 'a', 'n'}, {'t', 't', 'i'}, {'a', 'r', 'a'}}
	R, C = len(board), len(board[0])
	dict := []string{"eat", "rain", "in", "rat"}
	fmt.Println(maxWords(dict)) // 3
}
