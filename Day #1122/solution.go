// Day 1122 - Ghost: winning starting letters for player 1 under optimal play
// Trie + minimax over prefixes. A mover loses if every letter completes a word
// or leads to a winning position for the opponent. Time: O(total letters).
package main

import (
	"fmt"
	"sort"
	"strings"
)

type TrieNode struct {
	children map[rune]*TrieNode
	isWord   bool
}

func newNode() *TrieNode {
	return &TrieNode{children: make(map[rune]*TrieNode)}
}

func insert(root *TrieNode, w string) {
	node := root
	for _, ch := range w {
		if node.children[ch] == nil {
			node.children[ch] = newNode()
		}
		node = node.children[ch]
	}
	node.isWord = true
}

func canWin(node *TrieNode) bool {
	for _, child := range node.children {
		if child.isWord {
			continue
		}
		if !canWin(child) {
			return true
		}
	}
	return false
}

func main() {
	words := []string{"cat", "calf", "dog", "bear"}
	root := newNode()
	for _, w := range words {
		insert(root, w)
	}

	var res []string
	for ch, child := range root.children {
		if !child.isWord && !canWin(child) {
			res = append(res, string(ch))
		}
	}
	sort.Strings(res)
	fmt.Println("Winning starting letters:", strings.Join(res, " "))
}
