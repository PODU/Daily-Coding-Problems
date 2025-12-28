// Shortest unique prefix via trie of char frequency counts.
// Walk each word until count==1. Time O(total chars), Space O(total chars).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	cnt  int
	next map[rune]*Node
}

func newNode() *Node { return &Node{next: make(map[rune]*Node)} }

func insert(root *Node, w string) {
	cur := root
	for _, c := range w {
		if cur.next[c] == nil {
			cur.next[c] = newNode()
		}
		cur = cur.next[c]
		cur.cnt++
	}
}

func shortestPrefix(root *Node, w string) string {
	cur := root
	var pre strings.Builder
	for _, c := range w {
		cur = cur.next[c]
		pre.WriteRune(c)
		if cur.cnt == 1 {
			break
		}
	}
	return pre.String()
}

func main() {
	words := []string{"dog", "cat", "apple", "apricot", "fish"}
	root := newNode()
	for _, w := range words {
		insert(root, w)
	}
	res := make([]string, len(words))
	for i, w := range words {
		res[i] = shortestPrefix(root, w)
	}
	fmt.Println("[" + strings.Join(res, ", ") + "]")
}
