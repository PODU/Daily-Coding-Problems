// Day 724: Evaluate an arithmetic expression binary tree.
// Approach: Post-order recursion; leaves are ints, internal nodes are operators.
// Time: O(n), Space: O(h).
package main

import "fmt"

type Node struct {
	op          byte // 0 for leaf
	val         int
	left, right *Node
}

func leaf(v int) *Node              { return &Node{val: v} }
func inner(o byte, l, r *Node) *Node { return &Node{op: o, left: l, right: r} }

func eval(root *Node) float64 {
	if root.left == nil && root.right == nil {
		return float64(root.val)
	}
	l, r := eval(root.left), eval(root.right)
	switch root.op {
	case '+':
		return l + r
	case '-':
		return l - r
	case '*':
		return l * r
	case '/':
		return l / r
	}
	return 0
}

func main() {
	tree := inner('*',
		inner('+', leaf(3), leaf(2)),
		inner('+', leaf(4), leaf(5)))
	fmt.Println(int(eval(tree))) // 45
}
