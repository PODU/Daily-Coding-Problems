// Day 1425: evaluate an arithmetic expression binary tree (+,-,*,/ internal; ints at leaves).
// Approach: post-order recursion, combine children by operator. O(n) time, O(h) space.
package main

import "fmt"

type Node struct {
	Op          string // operator for internal nodes, "" for leaves
	Val         float64
	Left, Right *Node
}

func leaf(v float64) *Node { return &Node{Val: v} }

func eval(n *Node) float64 {
	if n.Left == nil && n.Right == nil {
		return n.Val
	}
	a, b := eval(n.Left), eval(n.Right)
	switch n.Op {
	case "+":
		return a + b
	case "-":
		return a - b
	case "*":
		return a * b
	default:
		return a / b
	}
}

func main() {
	root := &Node{Op: "*",
		Left:  &Node{Op: "+", Left: leaf(3), Right: leaf(2)},
		Right: &Node{Op: "+", Left: leaf(4), Right: leaf(5)}}
	fmt.Println(int(eval(root))) // 45
}
