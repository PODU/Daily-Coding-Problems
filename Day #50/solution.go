// Day 50: Evaluate arithmetic expression binary tree via post-order recursion.
// Time: O(n), Space: O(h).
package main

import "fmt"

type Node struct {
	op          byte // 0 for leaf
	val         float64
	left, right *Node
}

func leaf(v float64) *Node              { return &Node{val: v} }
func inner(op byte, l, r *Node) *Node   { return &Node{op: op, left: l, right: r} }

func eval(n *Node) float64 {
	if n.op == 0 {
		return n.val
	}
	a, b := eval(n.left), eval(n.right)
	switch n.op {
	case '+':
		return a + b
	case '-':
		return a - b
	case '*':
		return a * b
	case '/':
		return a / b
	}
	return 0
}

func main() {
	root := inner('*',
		inner('+', leaf(3), leaf(2)),
		inner('+', leaf(4), leaf(5)))
	fmt.Println(int(eval(root)))
}
