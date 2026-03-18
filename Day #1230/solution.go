// Trie with pass-through counts; for each word walk down until count==1 to get its shortest unique prefix.
// Time: O(total chars), Space: O(total chars).
package main

import (
	"fmt"
	"strings"
)

type Trie struct {
	next  map[rune]*Trie
	count int
}

func newTrie() *Trie {
	return &Trie{next: make(map[rune]*Trie)}
}

func insert(root *Trie, word string) {
	cur := root
	for _, c := range word {
		if cur.next[c] == nil {
			cur.next[c] = newTrie()
		}
		cur = cur.next[c]
		cur.count++
	}
}

func prefix(root *Trie, word string) string {
	cur := root
	var p strings.Builder
	for _, c := range word {
		cur = cur.next[c]
		p.WriteRune(c)
		if cur.count == 1 {
			break
		}
	}
	return p.String()
}

func main() {
	words := []string{"dog", "cat", "apple", "apricot", "fish"}
	root := newTrie()
	for _, w := range words {
		insert(root, w)
	}
	out := make([]string, len(words))
	for i, w := range words {
		out[i] = prefix(root, w)
	}
	fmt.Println("[" + strings.Join(out, ", ") + "]")
}
