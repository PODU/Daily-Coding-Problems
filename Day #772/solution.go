// Day 772: Boggle solver. Trie of dictionary + DFS from each cell over 8 neighbors,
// marking visited. O(W) to build trie; search bounded by trie/board size.
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Node struct {
	ch  [26]*Node
	end bool
}

func insert(root *Node, w string) {
	cur := root
	for i := 0; i < len(w); i++ {
		idx := w[i] - 'a'
		if cur.ch[idx] == nil {
			cur.ch[idx] = &Node{}
		}
		cur = cur.ch[idx]
	}
	cur.end = true
}

func dfs(b [][]byte, r, c int, node *Node, path []byte, out map[string]bool) {
	ch := b[r][c]
	if ch == '#' {
		return
	}
	nxt := node.ch[ch-'a']
	if nxt == nil {
		return
	}
	path = append(path, ch)
	if nxt.end {
		out[string(path)] = true
	}
	b[r][c] = '#'
	for dr := -1; dr <= 1; dr++ {
		for dc := -1; dc <= 1; dc++ {
			if dr == 0 && dc == 0 {
				continue
			}
			nr, nc := r+dr, c+dc
			if nr >= 0 && nr < len(b) && nc >= 0 && nc < len(b[0]) {
				dfs(b, nr, nc, nxt, path, out)
			}
		}
	}
	b[r][c] = ch
}

func solveBoggle(board, dict []string) []string {
	root := &Node{}
	for _, w := range dict {
		insert(root, w)
	}
	b := make([][]byte, len(board))
	for i, row := range board {
		b[i] = []byte(row)
	}
	out := map[string]bool{}
	for r := range b {
		for c := range b[0] {
			dfs(b, r, c, root, []byte{}, out)
		}
	}
	res := make([]string, 0, len(out))
	for w := range out {
		res = append(res, w)
	}
	sort.Strings(res)
	return res
}

func main() {
	board := []string{"oaan", "etae", "ihkr", "iflv"}
	dict := []string{"oath", "pea", "eat", "rain"}
	fmt.Println(strings.Join(solveBoggle(board, dict), " ")) // eat oath
}
