// Day 162: Shortest unique prefix via trie. Each node stores pass-through count;
// the shortest prefix with count 1 is unique. Time O(total chars), Space O(same).
package main

import "fmt"

type Node struct {
	count int
	child map[rune]*Node
}

func newNode() *Node { return &Node{child: make(map[rune]*Node)} }

func shortestUniquePrefixes(words []string) []string {
	root := newNode()
	for _, w := range words {
		cur := root
		for _, c := range w {
			if cur.child[c] == nil {
				cur.child[c] = newNode()
			}
			cur = cur.child[c]
			cur.count++
		}
	}
	result := make([]string, 0, len(words))
	for _, w := range words {
		cur := root
		prefix := ""
		for _, c := range w {
			cur = cur.child[c]
			prefix += string(c)
			if cur.count == 1 {
				break
			}
		}
		result = append(result, prefix)
	}
	return result
}

func main() {
	words := []string{"dog", "cat", "apple", "apricot", "fish"}
	for _, p := range shortestUniquePrefixes(words) {
		fmt.Println(p)
	}
}
