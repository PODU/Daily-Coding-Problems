// Day 609: Inorder successor in a BST using parent pointers.
// Approach: if right child exists, take its leftmost; else climb until coming from a left child. Time O(h).
package main

import "fmt"

type Node struct {
	val                 int
	left, right, parent *Node
}

func inorderSuccessor(node *Node) *Node {
	if node == nil {
		return nil
	}
	if node.right != nil {
		c := node.right
		for c.left != nil {
			c = c.left
		}
		return c
	}
	cur, p := node, node.parent
	for p != nil && cur == p.right {
		cur, p = p, p.parent
	}
	return p
}

func attach(parent, child *Node) *Node {
	if child != nil {
		child.parent = parent
	}
	return child
}

func main() {
	n10 := &Node{val: 10}
	n5 := &Node{val: 5}
	n30 := &Node{val: 30}
	n22 := &Node{val: 22}
	n35 := &Node{val: 35}
	n10.left = attach(n10, n5)
	n10.right = attach(n10, n30)
	n30.left = attach(n30, n22)
	n30.right = attach(n30, n35)

	s := inorderSuccessor(n22)
	if s != nil {
		fmt.Println(s.val) // 30
	} else {
		fmt.Println("null")
	}
}
