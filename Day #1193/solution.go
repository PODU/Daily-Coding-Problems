// Ghost word game: trie + game theory. canWin(node)=mover can force win.
// Winning start c: child not a word AND opponent (canWin(child)) cannot win.
// Time O(total chars), Space O(total chars).
// NOTE: README sample shows only "b" but "c" is also winning.
package main

import (
	"fmt"
	"sort"
	"strings"
)

type Node struct {
	ch     map[byte]*Node
	isWord bool
}

func newNode() *Node {
	return &Node{ch: make(map[byte]*Node)}
}

func insert(root *Node, w string) {
	cur := root
	for i := 0; i < len(w); i++ {
		c := w[i]
		if cur.ch[c] == nil {
			cur.ch[c] = newNode()
		}
		cur = cur.ch[c]
	}
	cur.isWord = true
}

// can the player about to move from this prefix force a win?
func canWin(node *Node) bool {
	for _, child := range node.ch {
		if child.isWord {
			continue // completing a word loses
		}
		if !canWin(child) { // opponent loses
			return true
		}
	}
	return false
}

func main() {
	dict := []string{"cat", "calf", "dog", "bear"}
	root := newNode()
	for _, w := range dict {
		insert(root, w)
	}

	var wins []string
	for c, child := range root.ch {
		if !child.isWord && !canWin(child) {
			wins = append(wins, string(c))
		}
	}
	sort.Strings(wins)
	fmt.Println(strings.Join(wins, " "))
}
