// Day 1475: Autocomplete via trie. Walk to prefix node, collect subtree words.
// Build O(total chars); query O(len(prefix) + matches). Space O(total chars).
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	next  map[byte]*Node
	order int
	word  string
}

func newNode() *Node { return &Node{next: make(map[byte]*Node), order: -1} }

type Trie struct{ root *Node }

func (t *Trie) insert(w string, order int) {
	node := t.root
	for i := 0; i < len(w); i++ {
		ch := w[i]
		if node.next[ch] == nil {
			node.next[ch] = newNode()
		}
		node = node.next[ch]
	}
	node.order = order
	node.word = w
}

func collect(n *Node, out *[]struct {
	order int
	word  string
}) {
	if n.order >= 0 {
		*out = append(*out, struct {
			order int
			word  string
		}{n.order, n.word})
	}
	for _, c := range n.next {
		collect(c, out)
	}
}

func (t *Trie) startsWith(prefix string) []string {
	node := t.root
	for i := 0; i < len(prefix); i++ {
		node = node.next[prefix[i]]
		if node == nil {
			return []string{}
		}
	}
	var out []struct {
		order int
		word  string
	}
	collect(node, &out)
	sort.Slice(out, func(i, j int) bool { return out[i].order < out[j].order })
	res := []string{}
	for _, p := range out {
		res = append(res, p.word)
	}
	return res
}

func main() {
	t := &Trie{root: newNode()}
	for i, w := range []string{"dog", "deer", "deal"} {
		t.insert(w, i)
	}
	fmt.Println(t.startsWith("de")) // [deer deal]
}
