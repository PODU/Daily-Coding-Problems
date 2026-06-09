// Validate BST via recursive inclusive min/max bounds (left<=root<=right). O(n) time, O(h) space.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val   int64
	left  *Node
	right *Node
}

func isValid(n *Node, lo, hi int64) bool {
	if n == nil {
		return true
	}
	if n.val < lo || n.val > hi {
		return false
	}
	return isValid(n.left, lo, n.val) && isValid(n.right, n.val, hi)
}

func validate(root *Node) bool {
	return isValid(root, math.MinInt64, math.MaxInt64)
}

func main() {
	// Valid BST: root 5, left 3 (2,4), right 8 (6,9)
	root := &Node{val: 5}
	root.left = &Node{val: 3}
	root.left.left = &Node{val: 2}
	root.left.right = &Node{val: 4}
	root.right = &Node{val: 8}
	root.right.left = &Node{val: 6}
	root.right.right = &Node{val: 9}

	// Invalid: root 5, left child 6
	bad := &Node{val: 5}
	bad.left = &Node{val: 6}

	if validate(root) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
	if validate(bad) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
