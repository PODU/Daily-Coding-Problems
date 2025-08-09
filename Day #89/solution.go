// Day 89: Validate BST via recursive [lo, hi] range check (left<=root<=right allowed).
// Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func valid(n *Node, lo, hi int64) bool {
	if n == nil {
		return true
	}
	v := int64(n.Val)
	if v < lo || v > hi {
		return false
	}
	return valid(n.Left, lo, v) && valid(n.Right, v, hi)
}

func isBST(root *Node) bool {
	return valid(root, math.MinInt64, math.MaxInt64)
}

func main() {
	a := &Node{Val: 5,
		Left:  &Node{Val: 3, Left: &Node{Val: 2}, Right: &Node{Val: 4}},
		Right: &Node{Val: 8}}
	fmt.Println(isBST(a)) // true

	b := &Node{Val: 5,
		Left:  &Node{Val: 3, Right: &Node{Val: 6}}, // invalid
		Right: &Node{Val: 8}}
	fmt.Println(isBST(b)) // false
}
