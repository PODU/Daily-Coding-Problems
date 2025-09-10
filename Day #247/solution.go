// Height-balanced binary tree check.
// Bottom-up DFS returning subtree height, or -1 if unbalanced. O(n) time, O(h) space.
package main

import "fmt"

type Node struct {
	val   int
	left  *Node
	right *Node
}

func check(root *Node) int {
	if root == nil {
		return 0
	}
	l := check(root.left)
	if l == -1 {
		return -1
	}
	r := check(root.right)
	if r == -1 {
		return -1
	}
	d := l - r
	if d < 0 {
		d = -d
	}
	if d > 1 {
		return -1
	}
	if l > r {
		return l + 1
	}
	return r + 1
}

func isBalanced(root *Node) bool {
	return check(root) != -1
}

func main() {
	a := &Node{val: 1}
	a.left = &Node{val: 2}
	a.right = &Node{val: 3}
	a.left.left = &Node{val: 4}

	b := &Node{val: 1}
	b.left = &Node{val: 2}
	b.left.left = &Node{val: 3}

	fmt.Printf("Balanced tree: %t\n", isBalanced(a))
	fmt.Printf("Unbalanced tree: %t\n", isBalanced(b))
}
