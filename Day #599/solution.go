// Day 599: Ghost game - find starting letters that guarantee player 1 a win.
// Approach: build a trie, minimax over it (landing on a word loses). Time O(total chars), Space O(trie).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Trie struct {
	ch     map[rune]*Trie
	isWord bool
}

func newTrie() *Trie { return &Trie{ch: make(map[rune]*Trie)} }

func insert(root *Trie, w string) {
	node := root
	for _, c := range w {
		if node.ch[c] == nil {
			node.ch[c] = newTrie()
		}
		node = node.ch[c]
	}
	node.isWord = true
}

// moverWins reports whether the player to move from node can force a loss on the opponent.
func moverWins(node *Trie) bool {
	for _, child := range node.ch {
		if child.isWord {
			continue
		}
		if !moverWins(child) {
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

	var keys []rune
	for c := range root.ch {
		keys = append(keys, c)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })

	var winning []string
	for _, c := range keys {
		child := root.ch[c]
		if !child.isWord && !moverWins(child) {
			winning = append(winning, string(c))
		}
	}
	fmt.Println(strings.Join(winning, " "))
}
