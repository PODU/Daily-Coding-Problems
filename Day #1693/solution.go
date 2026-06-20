// Ternary Search Tree: node has char + left/mid/right + isEnd. Compare char: <left, >right, ==mid & advance.
// Insert/search: O(L * log A) average where L=key length, A=alphabet size.
package main

import "fmt"

type Node struct {
	c                 byte
	isEnd             bool
	left, mid, right  *Node
}

func insert(node *Node, s string, i int) *Node {
	ch := s[i]
	if node == nil {
		node = &Node{c: ch}
	}
	if ch < node.c {
		node.left = insert(node.left, s, i)
	} else if ch > node.c {
		node.right = insert(node.right, s, i)
	} else if i+1 < len(s) {
		node.mid = insert(node.mid, s, i+1)
	} else {
		node.isEnd = true
	}
	return node
}

func search(node *Node, s string, i int) bool {
	if node == nil {
		return false
	}
	ch := s[i]
	if ch < node.c {
		return search(node.left, s, i)
	}
	if ch > node.c {
		return search(node.right, s, i)
	}
	if i+1 == len(s) {
		return node.isEnd
	}
	return search(node.mid, s, i+1)
}

func main() {
	var root *Node
	words := []string{"code", "cob", "be", "ax", "war", "we"}
	for _, w := range words {
		root = insert(root, w, 0)
	}

	queries := []string{"code", "cob", "cod", "war", "wa"}
	for _, q := range queries {
		fmt.Println(search(root, q, 0))
	}
}
