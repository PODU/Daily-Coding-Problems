// Day 935: Check if a binary tree is height-balanced.
// Bottom-up DFS returning height, -1 signals imbalance. Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

// Returns height if balanced, else -1.
func check(n *Node) int {
	if n == nil {
		return 0
	}
	l := check(n.left)
	if l == -1 {
		return -1
	}
	r := check(n.right)
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
		return 1 + l
	}
	return 1 + r
}

func isBalanced(root *Node) bool { return check(root) != -1 }

func main() {
	a := &Node{val: 1, left: &Node{val: 2, left: &Node{val: 4}}, right: &Node{val: 3}}
	fmt.Println(isBalanced(a)) // true

	b := &Node{val: 1, left: &Node{val: 2, left: &Node{val: 3, left: &Node{val: 4}}}}
	fmt.Println(isBalanced(b)) // false
}
