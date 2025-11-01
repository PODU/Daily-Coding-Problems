// Boggle solver: build a trie from the dictionary, DFS each cell over 8 neighbors
// (each cell used once per path), collect words present in the trie.
// Time: O(cells * 8^L) bounded by trie depth; Space: O(total dictionary chars).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Trie struct {
	next [26]*Trie
	word bool
}

var (
	board [][]byte
	R, C  int
	found map[string]bool
)

func dfs(r, c int, node *Trie, cur string, used [][]bool) {
	if r < 0 || r >= R || c < 0 || c >= C || used[r][c] {
		return
	}
	ch := board[r][c]
	nxt := node.next[ch-'a']
	if nxt == nil {
		return
	}
	cur += string(ch)
	used[r][c] = true
	if nxt.word {
		found[cur] = true
	}
	for dr := -1; dr <= 1; dr++ {
		for dc := -1; dc <= 1; dc++ {
			if dr != 0 || dc != 0 {
				dfs(r+dr, c+dc, nxt, cur, used)
			}
		}
	}
	used[r][c] = false
}

func main() {
	rows := []string{"oaan", "etae", "ihkr", "iflv"}
	board = make([][]byte, len(rows))
	for i, s := range rows {
		board[i] = []byte(s)
	}
	dict := []string{"oath", "pea", "eat", "rain"}
	R, C = len(board), len(board[0])
	found = map[string]bool{}

	root := &Trie{}
	for _, w := range dict {
		node := root
		for i := 0; i < len(w); i++ {
			idx := w[i] - 'a'
			if node.next[idx] == nil {
				node.next[idx] = &Trie{}
			}
			node = node.next[idx]
		}
		node.word = true
	}

	used := make([][]bool, R)
	for i := range used {
		used[i] = make([]bool, C)
	}
	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			dfs(r, c, root, "", used)
		}
	}

	res := make([]string, 0, len(found))
	for w := range found {
		res = append(res, w)
	}
	sort.Strings(res)
	fmt.Println("[" + strings.Join(res, ", ") + "]")
}
