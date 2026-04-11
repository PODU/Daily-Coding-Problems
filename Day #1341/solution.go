// Check height-balanced binary tree via bottom-up DFS; -1 sentinel marks unbalanced.
// Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func height(root *Node) int {
	if root == nil {
		return 0
	}
	l := height(root.Left)
	if l == -1 {
		return -1
	}
	r := height(root.Right)
	if r == -1 {
		return -1
	}
	diff := l - r
	if diff < 0 {
		diff = -diff
	}
	if diff > 1 {
		return -1
	}
	if l > r {
		return l + 1
	}
	return r + 1
}

func isBalanced(root *Node) bool {
	return height(root) != -1
}

func main() {
	// Balanced tree [1,2,3,4,5,null,6]
	a := &Node{Val: 1,
		Left:  &Node{Val: 2, Left: &Node{Val: 4}, Right: &Node{Val: 5}},
		Right: &Node{Val: 3, Right: &Node{Val: 6}}}
	fmt.Printf("Balanced: %t\n", isBalanced(a))

	// Skewed tree 1 -> 2 -> 3
	b := &Node{Val: 1, Left: &Node{Val: 2, Left: &Node{Val: 3}}}
	fmt.Printf("Balanced: %t\n", isBalanced(b))
}
