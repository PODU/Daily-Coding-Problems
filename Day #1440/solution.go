// Day 1440: Ternary search tree with insert and search.
// Approach: each node stores a char + left/mid/right; mid advances the word.
// Time: insert/search O(L * log A) avg, Space: O(total chars).
package main

import "fmt"

type Node struct {
	c                 byte
	isEnd             bool
	left, mid, right  *Node
}

func insert(root *Node, word string, i int) *Node {
	if i >= len(word) {
		return root
	}
	ch := word[i]
	if root == nil {
		root = &Node{c: ch}
	}
	if ch < root.c {
		root.left = insert(root.left, word, i)
	} else if ch > root.c {
		root.right = insert(root.right, word, i)
	} else {
		if i+1 == len(word) {
			root.isEnd = true
		} else {
			root.mid = insert(root.mid, word, i+1)
		}
	}
	return root
}

func search(root *Node, word string, i int) bool {
	if root == nil || i >= len(word) {
		return false
	}
	ch := word[i]
	if ch < root.c {
		return search(root.left, word, i)
	}
	if ch > root.c {
		return search(root.right, word, i)
	}
	if i+1 == len(word) {
		return root.isEnd
	}
	return search(root.mid, word, i+1)
}

func main() {
	var root *Node
	for _, w := range []string{"code", "cob", "be", "ax", "war", "we"} {
		root = insert(root, w, 0)
	}
	fmt.Println(search(root, "code", 0)) // true
	fmt.Println(search(root, "cob", 0))  // true
	fmt.Println(search(root, "we", 0))   // true
	fmt.Println(search(root, "cod", 0))  // false
	fmt.Println(search(root, "cat", 0))  // false
}
