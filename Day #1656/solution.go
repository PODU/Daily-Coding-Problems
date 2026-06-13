// Trie autocomplete: insert words, DFS from prefix node in child-insertion order
// (ordered key slice). O(total chars) build, O(matches) query; O(total chars) space.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	children map[rune]*Node
	order    []rune
	word     string
	isWord   bool
}

func newNode() *Node { return &Node{children: map[rune]*Node{}} }

type Trie struct{ root *Node }

func (t *Trie) insert(w string) {
	n := t.root
	for _, c := range w {
		if _, ok := n.children[c]; !ok {
			n.children[c] = newNode()
			n.order = append(n.order, c)
		}
		n = n.children[c]
	}
	n.word = w
	n.isWord = true
}

func (t *Trie) dfs(n *Node, out *[]string) {
	if n.isWord {
		*out = append(*out, n.word)
	}
	for _, c := range n.order {
		t.dfs(n.children[c], out)
	}
}

func (t *Trie) autocomplete(s string) []string {
	n := t.root
	for _, c := range s {
		nx, ok := n.children[c]
		if !ok {
			return []string{}
		}
		n = nx
	}
	out := []string{}
	t.dfs(n, &out)
	return out
}

func main() {
	t := &Trie{root: newNode()}
	for _, w := range []string{"dog", "deer", "deal"} {
		t.insert(w)
	}
	res := t.autocomplete("de")
	fmt.Println("[" + strings.Join(res, ", ") + "]")
}
