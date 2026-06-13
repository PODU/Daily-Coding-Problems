// Evaluate arithmetic expression tree: recurse, combining children by operator
// at each internal node; leaves are integers. Time O(n), Space O(h) recursion.
package main

import "fmt"

type Node struct {
	isLeaf bool
	val    int64
	op     byte
	left   *Node
	right  *Node
}

func leaf(v int64) *Node {
	return &Node{isLeaf: true, val: v}
}

func opNode(o byte, l, r *Node) *Node {
	return &Node{isLeaf: false, op: o, left: l, right: r}
}

func eval(n *Node) int64 {
	if n.isLeaf {
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
	left := opNode('+', leaf(3), leaf(2))
	right := opNode('+', leaf(4), leaf(5))
	root := opNode('*', left, right)
	fmt.Println(eval(root))
}
