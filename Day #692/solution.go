// Day 692: Autocomplete - return all dictionary strings having s as a prefix.
// Approach: Trie. Insert words O(total chars); query walks prefix then DFS to
// collect matches. Query O(|s| + #matches * len).
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	ch  map[rune]*Node
	end bool
}

func newNode() *Node { return &Node{ch: make(map[rune]*Node)} }

type Trie struct{ root *Node }

func (t *Trie) insert(w string) {
	cur := t.root
	for _, c := range w {
		if cur.ch[c] == nil {
			cur.ch[c] = newNode()
		}
		cur = cur.ch[c]
	}
	cur.end = true
}

func (t *Trie) dfs(n *Node, cur string, out *[]string) {
	if n.end {
		*out = append(*out, cur)
	}
	for c, nx := range n.ch {
		t.dfs(nx, cur+string(c), out)
	}
}

func (t *Trie) autocomplete(s string) []string {
	cur := t.root
	for _, c := range s {
		if cur.ch[c] == nil {
			return []string{}
		}
		cur = cur.ch[c]
	}
	out := []string{}
	t.dfs(cur, s, &out)
	return out
}

func main() {
	t := &Trie{root: newNode()}
	for _, w := range []string{"dog", "deer", "deal"} {
		t.insert(w)
	}
	res := t.autocomplete("de")
	sort.Strings(res)
	fmt.Println(res) // [deal deer]
}
