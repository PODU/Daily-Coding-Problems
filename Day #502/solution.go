// Height-balanced check via bottom-up recursion returning height, -1 sentinel = unbalanced.
// Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func height(n *Node) int {
	if n == nil {
		return 0
	}
	lh := height(n.left)
	if lh == -1 {
		return -1
	}
	rh := height(n.right)
	if rh == -1 {
		return -1
	}
	diff := lh - rh
	if diff < 0 {
		diff = -diff
	}
	if diff > 1 {
		return -1
	}
	if lh > rh {
		return lh + 1
	}
	return rh + 1
}

func isBalanced(root *Node) bool {
	return height(root) != -1
}

func main() {
	// Balanced tree
	a := &Node{val: 1,
		left:  &Node{val: 2, left: &Node{val: 4}},
		right: &Node{val: 3}}

	// Unbalanced left-leaning chain 1 -> 2 -> 3 -> 4
	b := &Node{val: 1,
		left: &Node{val: 2,
			left: &Node{val: 3,
				left: &Node{val: 4}}}}

	fmt.Println(isBalanced(a))
	fmt.Println(isBalanced(b))
}
