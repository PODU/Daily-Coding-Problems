// Day 1474: Boggle solver. Trie of dictionary + DFS from each cell over
// 8-adjacent neighbors (no reuse), pruned by trie prefixes.
// Time O(R*C*8^L) worst case; Space O(total dict chars).
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	next map[byte]*Node
	word string
}

func newNode() *Node { return &Node{next: make(map[byte]*Node)} }

func dfs(b [][]byte, r, c int, node *Node, found map[string]bool) {
	ch := b[r][c]
	if ch == '*' {
		return
	}
	nxt, ok := node.next[ch]
	if !ok {
		return
	}
	if nxt.word != "" {
		found[nxt.word] = true
	}
	b[r][c] = '*'
	for dr := -1; dr <= 1; dr++ {
		for dc := -1; dc <= 1; dc++ {
			if dr == 0 && dc == 0 {
				continue
			}
			nr, nc := r+dr, c+dc
			if nr >= 0 && nr < len(b) && nc >= 0 && nc < len(b[0]) {
				dfs(b, nr, nc, nxt, found)
			}
		}
	}
	b[r][c] = ch
}

func main() {
	board := [][]byte{
		[]byte("oaan"),
		[]byte("etae"),
		[]byte("ihkr"),
		[]byte("iflv"),
	}
	words := []string{"oath", "pea", "eat", "rain"}

	root := newNode()
	for _, w := range words {
		node := root
		for i := 0; i < len(w); i++ {
			ch := w[i]
			if node.next[ch] == nil {
				node.next[ch] = newNode()
			}
			node = node.next[ch]
		}
		node.word = w
	}

	found := map[string]bool{}
	for r := range board {
		for c := range board[0] {
			dfs(board, r, c, root, found)
		}
	}
	res := []string{}
	for w := range found {
		res = append(res, w)
	}
	sort.Strings(res)
	fmt.Println(res) // [eat oath]
}
