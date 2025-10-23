// Day 482: BST range sum [a,b] inclusive via DFS with pruning.
// Skip left subtree if node<a, skip right if node>b. Time O(n) worst, O(k+h) typical; Space O(h).
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
	if root.val < a {
		return rangeSum(root.right, a, b)
	}
	if root.val > b {
		return rangeSum(root.left, a, b)
	}
	return root.val + rangeSum(root.left, a, b) + rangeSum(root.right, a, b)
}

func main() {
	root := &Node{5,
		&Node{3, &Node{2, nil, nil}, &Node{4, nil, nil}},
		&Node{8, &Node{6, nil, nil}, &Node{10, nil, nil}}}
	fmt.Println(rangeSum(root, 4, 9)) // 23
}
