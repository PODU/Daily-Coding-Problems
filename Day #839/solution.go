// Day 839: Max number of dictionary words packed on an NxN board.
// For each word collect all valid adjacent-path placements (DFS), then backtrack
// over words choosing a disjoint set to maximize the count.
// Time O(exponential worst-case) on small boards; placement search bounded by board size.
package main

import "fmt"

var R, C int
var best int
var wordPlacements [][]int64

func dfs(mat []string, word string, r, c, idx int, used int64, placements map[int64]bool) {
	if mat[r][c] != word[idx] {
		return
	}
	used |= 1 << uint(r*C+c)
	if idx == len(word)-1 {
		placements[used] = true
		return
	}
	dr := []int{1, -1, 0, 0}
	dc := []int{0, 0, 1, -1}
	for d := 0; d < 4; d++ {
		nr, nc := r+dr[d], c+dc[d]
		if nr >= 0 && nr < R && nc >= 0 && nc < C && used&(1<<uint(nr*C+nc)) == 0 {
			dfs(mat, word, nr, nc, idx+1, used, placements)
		}
	}
}

func findPlacements(mat []string, word string) []int64 {
	placements := make(map[int64]bool)
	for i := 0; i < R; i++ {
		for j := 0; j < C; j++ {
			dfs(mat, word, i, j, 0, 0, placements)
		}
	}
	res := make([]int64, 0, len(placements))
	for k := range placements {
		res = append(res, k)
	}
	return res
}

func backtrack(i int, occupied int64, count int) {
	if count > best {
		best = count
	}
	if i == len(wordPlacements) {
		return
	}
	backtrack(i+1, occupied, count) // skip
	for _, tiles := range wordPlacements[i] {
		if occupied&tiles == 0 {
			backtrack(i+1, occupied|tiles, count+1)
		}
	}
}

func maxWords(mat []string, dict []string) int {
	wordPlacements = nil
	for _, w := range dict {
		p := findPlacements(mat, w)
		if len(p) > 0 {
			wordPlacements = append(wordPlacements, p)
		}
	}
	best = 0
	backtrack(0, 0, 0)
	return best
}

func main() {
	mat := []string{"ean", "tti", "ara"}
	R = len(mat)
	C = len(mat[0])
	dict := []string{"eat", "rain", "in", "rat"}
	fmt.Println(maxWords(mat, dict))
}
