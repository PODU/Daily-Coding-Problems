// Day 1606: Check if a binary tree is height-balanced.
// Bottom-up recursion returning height, -1 if unbalanced. Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func check(root *Node) int { // height, or -1 if unbalanced
	if root == nil {
		return 0
	}
	l := check(root.left)
	if l == -1 {
		return -1
	}
	r := check(root.right)
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
		return l + 1
	}
	return r + 1
}

func isBalanced(root *Node) bool { return check(root) != -1 }

func main() {
	root := &Node{val: 1,
		left:  &Node{val: 2, left: &Node{val: 4}},
		right: &Node{val: 3}}
	if isBalanced(root) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
