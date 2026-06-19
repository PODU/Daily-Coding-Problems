// Boggle solver: build a trie from the dictionary, DFS 8-directionally from each
// cell following trie edges with a visited mask. O(cells * 8^L) worst, pruned by trie.
package main

import (
	"fmt"
	"sort"
)

type TrieNode struct {
	ch  [26]*TrieNode
	end bool
}

func main() {
	rows := []string{"oaan", "etae", "ihkr", "iflv"}
	dict := []string{"oath", "pea", "eat", "rain"}

	root := &TrieNode{}
	for _, w := range dict {
		nd := root
		for _, c := range w {
			i := c - 'a'
			if nd.ch[i] == nil {
				nd.ch[i] = &TrieNode{}
			}
			nd = nd.ch[i]
		}
		nd.end = true
	}

	R := len(rows)
	C := len(rows[0])
	grid := make([][]byte, R)
	for i := range rows {
		grid[i] = []byte(rows[i])
	}
	vis := make([][]bool, R)
	for i := range vis {
		vis[i] = make([]bool, C)
	}
	foundSet := map[string]bool{}

	var dfs func(r, c int, node *TrieNode, path []byte)
	dfs = func(r, c int, node *TrieNode, path []byte) {
		idx := grid[r][c] - 'a'
		nxt := node.ch[idx]
		if nxt == nil {
			return
		}
		path = append(path, grid[r][c])
		if nxt.end {
			foundSet[string(path)] = true
		}
		vis[r][c] = true
		for dr := -1; dr <= 1; dr++ {
			for dc := -1; dc <= 1; dc++ {
				if dr == 0 && dc == 0 {
					continue
				}
				nr, nc := r+dr, c+dc
				if nr >= 0 && nr < R && nc >= 0 && nc < C && !vis[nr][nc] {
					dfs(nr, nc, nxt, path)
				}
			}
		}
		vis[r][c] = false
	}

	for r := 0; r < R; r++ {
		for c := 0; c < C; c++ {
			dfs(r, c, root, []byte{})
		}
	}

	var result []string
	for w := range foundSet {
		result = append(result, w)
	}
	sort.Strings(result)
	for _, w := range result {
		fmt.Println(w)
	}
}
