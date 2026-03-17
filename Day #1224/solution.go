// DFS with BST pruning: skip left if val<a, skip right if val>b. O(n) worst.
// O(n) time worst, O(h) space (recursion).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func rangeSumBST(node *Node, a, b int) int {
	if node == nil {
		return 0
	}
	if node.Val < a {
		return rangeSumBST(node.Right, a, b)
	}
	if node.Val > b {
		return rangeSumBST(node.Left, a, b)
	}
	return node.Val + rangeSumBST(node.Left, a, b) + rangeSumBST(node.Right, a, b)
}

func main() {
	root := &Node{
		Val:   5,
		Left:  &Node{Val: 3, Left: &Node{Val: 2}, Right: &Node{Val: 4}},
		Right: &Node{Val: 8, Left: &Node{Val: 6}, Right: &Node{Val: 10}},
	}
	fmt.Println(rangeSumBST(root, 4, 9))
}
