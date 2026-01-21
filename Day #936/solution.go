// Day 936: Sum of BST values within inclusive range [a,b], pruning branches out of range.
// Time O(n) worst, O(h + k) typical, Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func rangeSum(n *Node, a, b int) int {
	if n == nil {
		return 0
	}
	if n.val < a {
		return rangeSum(n.right, a, b)
	}
	if n.val > b {
		return rangeSum(n.left, a, b)
	}
	return n.val + rangeSum(n.left, a, b) + rangeSum(n.right, a, b)
}

func main() {
	root := &Node{val: 5,
		left:  &Node{val: 3, left: &Node{val: 2}, right: &Node{val: 4}},
		right: &Node{val: 8, left: &Node{val: 6}, right: &Node{val: 10}}}
	fmt.Println(rangeSum(root, 4, 9)) // 23
}
