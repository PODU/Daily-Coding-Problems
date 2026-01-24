// Day 949: Autocomplete - return all words having query as a prefix, using a Trie.
// Build O(total chars); query O(|s| + matches). Insertion order preserved.
package main

import (
	"fmt"
	"strings"
)

type node struct {
	next map[rune]*node
	ids  []int
}

func newNode() *node { return &node{next: make(map[rune]*node)} }

type trie struct {
	root  *node
	words []string
}

func (t *trie) insert(w string) {
	idx := len(t.words)
	t.words = append(t.words, w)
	cur := t.root
	for _, c := range w {
		if cur.next[c] == nil {
			cur.next[c] = newNode()
		}
		cur = cur.next[c]
		cur.ids = append(cur.ids, idx)
	}
}

func (t *trie) withPrefix(s string) []string {
	cur := t.root
	for _, c := range s {
		if cur.next[c] == nil {
			return nil
		}
		cur = cur.next[c]
	}
	res := make([]string, len(cur.ids))
	for i, id := range cur.ids {
		res[i] = t.words[id]
	}
	return res
}

func main() {
	t := &trie{root: newNode()}
	for _, w := range []string{"dog", "deer", "deal"} {
		t.insert(w)
	}
	fmt.Println("[" + strings.Join(t.withPrefix("de"), ", ") + "]") // [deer, deal]
}
