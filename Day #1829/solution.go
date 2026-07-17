// Ghost: build a trie, solve the game bottom-up. A mover wins if some child is
// not a word and is a losing position for the opponent. O(total chars).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Trie struct {
	ch   map[byte]*Trie
	word bool
}

func newTrie() *Trie { return &Trie{ch: map[byte]*Trie{}} }

func insert(root *Trie, w string) {
	cur := root
	for i := 0; i < len(w); i++ {
		c := w[i]
		if cur.ch[c] == nil {
			cur.ch[c] = newTrie()
		}
		cur = cur.ch[c]
	}
	cur.word = true
}

func canWin(node *Trie) bool {
	for _, child := range node.ch {
		if child.word {
			continue
		}
		if !canWin(child) {
			return true
		}
	}
	return false
}

func main() {
	dict := []string{"cat", "calf", "dog", "bear"}
	root := newTrie()
	for _, w := range dict {
		insert(root, w)
	}
	var wins []string
	for c, child := range root.ch {
		if !child.word && !canWin(child) {
			wins = append(wins, string(c))
		}
	}
	sort.Strings(wins)
	fmt.Println(strings.Join(wins, ", "))
}
