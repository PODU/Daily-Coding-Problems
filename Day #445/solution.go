// Day 445: Prune a 0/1 binary tree, removing all-zero subtrees.
// Postorder recursion, O(n) time, O(h) space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func prune(n *Node) *Node {
	if n == nil {
		return nil
	}
	n.left = prune(n.left)
	n.right = prune(n.right)
	if n.val == 0 && n.left == nil && n.right == nil {
		return nil
	}
	return n
}

func show(n *Node, prefix, tag string) {
	if n == nil {
		return
	}
	fmt.Printf("%s%s%d\n", prefix, tag, n.val)
	show(n.left, prefix+"  ", "L-- ")
	show(n.right, prefix+"  ", "R-- ")
}

func main() {
	root := &Node{val: 0,
		left: &Node{val: 1},
		right: &Node{val: 0,
			left:  &Node{val: 1, left: &Node{val: 0}, right: &Node{val: 0}},
			right: &Node{val: 0}}}
	root = prune(root)
	show(root, "", "")
	// 0
	//   L-- 1
	//   R-- 0
	//     L-- 1
}
