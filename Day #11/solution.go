// Autocomplete via Trie: insert all words, walk to prefix node, DFS collect.
// Build: O(total chars); query: O(|prefix| + matches). Results in insertion order.
package main

import (
	"fmt"
	"sort"
)

type TrieNode struct {
	ch    map[rune]*TrieNode
	order int
}

func newNode() *TrieNode { return &TrieNode{ch: map[rune]*TrieNode{}, order: -1} }

type Trie struct {
	root    *TrieNode
	counter int
}

func NewTrie() *Trie { return &Trie{root: newNode()} }

func (t *Trie) insert(w string) {
	cur := t.root
	for _, c := range w {
		if cur.ch[c] == nil {
			cur.ch[c] = newNode()
		}
		cur = cur.ch[c]
	}
	cur.order = t.counter
	t.counter++
}

func (t *Trie) autocomplete(prefix string) []string {
	cur := t.root
	for _, c := range prefix {
		if cur.ch[c] == nil {
			return []string{}
		}
		cur = cur.ch[c]
	}
	type entry struct {
		order int
		word  string
	}
	var found []entry
	var dfs func(n *TrieNode, buf string)
	dfs = func(n *TrieNode, buf string) {
		if n.order >= 0 {
			found = append(found, entry{n.order, buf})
		}
		keys := make([]rune, 0, len(n.ch))
		for c := range n.ch {
			keys = append(keys, c)
		}
		sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })
		for _, c := range keys {
			dfs(n.ch[c], buf+string(c))
		}
	}
	dfs(cur, prefix)
	sort.Slice(found, func(i, j int) bool { return found[i].order < found[j].order })
	res := make([]string, len(found))
	for i, e := range found {
		res[i] = e.word
	}
	return res
}

func main() {
	t := NewTrie()
	for _, w := range []string{"dog", "deer", "deal"} {
		t.insert(w)
	}
	fmt.Println(t.autocomplete("de"))
}
