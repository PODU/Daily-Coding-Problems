// Range Sum of BST. Pruned DFS using BST property. Time O(n) worst, Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func rangeSum(root *Node, a, b int) int {
	if root == nil {
		return 0
	}
	s := 0
	if root.val >= a && root.val <= b {
		s += root.val
	}
	if root.val > a {
		s += rangeSum(root.left, a, b)
	}
	if root.val < b {
		s += rangeSum(root.right, a, b)
	}
	return s
}

func main() {
	root := &Node{val: 5}
	root.left = &Node{val: 3}
	root.left.left = &Node{val: 2}
	root.left.right = &Node{val: 4}
	root.right = &Node{val: 8}
	root.right.left = &Node{val: 6}
	root.right.right = &Node{val: 10}
	fmt.Println(rangeSum(root, 4, 9))
}
