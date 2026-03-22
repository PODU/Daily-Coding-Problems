// Validate BST with inclusive bounds (left <= node <= right) via recursive range check. O(n) time, O(h) space.
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

func isValid(node *Node, lo, hi int64) bool {
	if node == nil {
		return true
	}
	if node.val < lo || node.val > hi {
		return false
	}
	return isValid(node.left, lo, node.val) && isValid(node.right, node.val, hi)
}

func isValidBST(root *Node) bool {
	return isValid(root, math.MinInt64, math.MaxInt64)
}

func main() {
	// Valid tree: root 5, left 3 (2,5), right 8 (5,12)
	root := &Node{val: 5}
	root.left = &Node{val: 3}
	root.left.left = &Node{val: 2}
	root.left.right = &Node{val: 5}
	root.right = &Node{val: 8}
	root.right.left = &Node{val: 5}
	root.right.right = &Node{val: 12}
	if isValidBST(root) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}

	// Invalid tree: root 5, left 6 (6 > 5 violates)
	bad := &Node{val: 5}
	bad.left = &Node{val: 6}
	bad.right = &Node{val: 8}
	if isValidBST(bad) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
