// Subtree check: for each node of s, test identical-tree with t. Time O(m*n), Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func sameTree(a, b *Node) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.val == b.val && sameTree(a.left, b.left) && sameTree(a.right, b.right)
}

func isSubtree(s, t *Node) bool {
	if s == nil {
		return false
	}
	if sameTree(s, t) {
		return true
	}
	return isSubtree(s.left, t) || isSubtree(s.right, t)
}

func main() {
	s := &Node{3,
		&Node{4, &Node{1, nil, nil}, &Node{2, nil, nil}},
		&Node{5, nil, nil}}
	t := &Node{4, &Node{1, nil, nil}, &Node{2, nil, nil}}
	fmt.Println(isSubtree(s, t))
}
