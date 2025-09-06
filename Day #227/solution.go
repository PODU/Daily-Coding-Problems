// Boggle solver: build a Trie of the dictionary, DFS from every cell over 8 neighbours.
// Time: O(cells * 8^L) bounded by Trie; Space: O(dictionary size).
package main

import (
	"fmt"
	"sort"
)

type Trie struct {
	next map[byte]*Trie
	word string
}

func newTrie() *Trie { return &Trie{next: map[byte]*Trie{}} }

func boggle(board [][]byte, dict []string) []string {
	root := newTrie()
	for _, w := range dict {
		n := root
		for i := 0; i < len(w); i++ {
			if n.next[w[i]] == nil {
				n.next[w[i]] = newTrie()
			}
			n = n.next[w[i]]
		}
		n.word = w
	}
	rows, cols := len(board), len(board[0])
	found := map[string]bool{}
	var dfs func(r, c int, node *Trie)
	dfs = func(r, c int, node *Trie) {
		ch := board[r][c]
		nx := node.next[ch]
		if nx == nil {
			return
		}
		if nx.word != "" {
			found[nx.word] = true
		}
		board[r][c] = '#'
		for dr := -1; dr <= 1; dr++ {
			for dc := -1; dc <= 1; dc++ {
				if dr == 0 && dc == 0 {
					continue
				}
				nr, nc := r+dr, c+dc
				if nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr][nc] != '#' {
					dfs(nr, nc, nx)
				}
			}
		}
		board[r][c] = ch
	}
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			dfs(r, c, root)
		}
	}
	res := []string{}
	for w := range found {
		res = append(res, w)
	}
	sort.Strings(res)
	return res
}

func main() {
	board := [][]byte{
		[]byte("oaan"),
		[]byte("etae"),
		[]byte("ihkr"),
		[]byte("iflv")}
	dict := []string{"oath", "pea", "eat", "rain"}
	res := boggle(board, dict)
	fmt.Print("[")
	for i, w := range res {
		fmt.Printf("'%s'", w)
		if i+1 < len(res) {
			fmt.Print(", ")
		}
	}
	fmt.Println("]")
}
