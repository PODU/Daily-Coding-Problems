// Day 1001: Validate a binary search tree.
// Recurse carrying an allowed [low, high] range; left key <= root <= right key.
// O(n) time, O(h) space.
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

func isBst(node *Node, low, high int) bool {
	if node == nil {
		return true
	}
	if node.val < low || node.val > high {
		return false
	}
	return isBst(node.left, low, node.val) && isBst(node.right, node.val, high)
}

func main() {
	valid := &Node{5,
		&Node{3, &Node{2, nil, nil}, &Node{4, nil, nil}},
		&Node{8, &Node{6, nil, nil}, &Node{9, nil, nil}}}
	invalid := &Node{5, &Node{6, nil, nil}, &Node{8, nil, nil}}
	fmt.Println(isBst(valid, math.MinInt, math.MaxInt))   // true
	fmt.Println(isBst(invalid, math.MinInt, math.MaxInt)) // false
}
