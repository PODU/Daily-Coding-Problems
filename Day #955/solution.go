// Day 955: evaluate an arithmetic expression binary tree (leaves=ints, nodes=+ - * /).
// Recursive post-order evaluation. Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	op          byte // 0 for leaf
	val         float64
	left, right *Node
}

func leaf(v float64) *Node { return &Node{val: v} }
func op(o byte, l, r *Node) *Node { return &Node{op: o, left: l, right: r} }

func eval(n *Node) float64 {
	if n.left == nil && n.right == nil {
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
	default:
		return a / b
	}
}

func main() {
	root := op('*',
		op('+', leaf(3), leaf(2)),
		op('+', leaf(4), leaf(5)))
	fmt.Println(int64(eval(root))) // 45
}
