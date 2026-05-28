// Shortest unique prefix via Trie storing char frequency counts. O(total chars) time & space.
package main

import "fmt"

type Node struct {
	ch  map[rune]*Node
	cnt int
}

func newNode() *Node { return &Node{ch: make(map[rune]*Node)} }

func main() {
	words := []string{"dog", "cat", "apple", "apricot", "fish"}
	root := newNode()
	for _, w := range words {
		cur := root
		for _, c := range w {
			if cur.ch[c] == nil {
				cur.ch[c] = newNode()
			}
			cur = cur.ch[c]
			cur.cnt++
		}
	}
	for _, w := range words {
		cur := root
		var pre []rune
		for _, c := range w {
			cur = cur.ch[c]
			pre = append(pre, c)
			if cur.cnt == 1 {
				break
			}
		}
		fmt.Println(string(pre))
	}
}
