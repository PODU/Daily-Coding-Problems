// Trie with per-node pass counts; shortest unique prefix = up to first node count==1. Time O(total chars).
package main

import "fmt"

type Node struct {
	next  map[rune]*Node
	count int
}

func newNode() *Node { return &Node{next: make(map[rune]*Node)} }

func shortestUniquePrefixes(words []string) []string {
	root := newNode()
	for _, w := range words {
		cur := root
		for _, c := range w {
			if cur.next[c] == nil {
				cur.next[c] = newNode()
			}
			cur = cur.next[c]
			cur.count++
		}
	}
	res := make([]string, 0, len(words))
	for _, w := range words {
		cur := root
		pre := ""
		for _, c := range w {
			cur = cur.next[c]
			pre += string(c)
			if cur.count == 1 {
				break
			}
		}
		res = append(res, pre)
	}
	return res
}

func main() {
	words := []string{"dog", "cat", "apple", "apricot", "fish"}
	fmt.Println(shortestUniquePrefixes(words))
}
