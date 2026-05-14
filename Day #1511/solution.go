// Range Sum of BST via DFS with BST pruning (skip left if node<a, right if node>b).
// O(n) worst-case time, O(h) recursion space.
package main

import "fmt"

type Node struct {
	Val   int
	Left  *Node
	Right *Node
}

func rangeSum(root *Node, a, b int) int {
	if root == nil {
		return 0
	}
	if root.Val < a {
		return rangeSum(root.Right, a, b)
	}
	if root.Val > b {
		return rangeSum(root.Left, a, b)
	}
	return root.Val + rangeSum(root.Left, a, b) + rangeSum(root.Right, a, b)
}

func main() {
	root := &Node{Val: 5}
	root.Left = &Node{Val: 3}
	root.Right = &Node{Val: 8}
	root.Left.Left = &Node{Val: 2}
	root.Left.Right = &Node{Val: 4}
	root.Right.Left = &Node{Val: 6}
	root.Right.Right = &Node{Val: 10}
	fmt.Println(rangeSum(root, 4, 9))
}
