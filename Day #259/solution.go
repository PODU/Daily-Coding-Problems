// Day 259: Ghost word game. Build a trie of the dictionary. A starting letter is
// guaranteed safe for player 1 iff every word in its subtree has even length, so
// the opponent is always the one forced to complete a word. Trie DFS, O(total chars).
package main

import (
	"fmt"
	"sort"
)

type T struct {
	kids map[byte]*T
	word bool
}

func newT() *T { return &T{kids: map[byte]*T{}} }

func insert(root *T, w string) {
	n := root
	for i := 0; i < len(w); i++ {
		c := w[i]
		if n.kids[c] == nil {
			n.kids[c] = newT()
		}
		n = n.kids[c]
	}
	n.word = true
}

func allEven(n *T, depth int) bool {
	if n.word && depth%2 != 0 {
		return false
	}
	for _, ch := range n.kids {
		if !allEven(ch, depth+1) {
			return false
		}
	}
	return true
}

func main() {
	words := []string{"cat", "calf", "dog", "bear"}
	root := newT()
	for _, w := range words {
		insert(root, w)
	}

	var keys []byte
	for c := range root.kids {
		keys = append(keys, c)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i] < keys[j] })

	res := ""
	for _, c := range keys {
		if allEven(root.kids[c], 1) {
			res += string(c)
		}
	}
	fmt.Println(res)
}
