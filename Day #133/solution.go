// Day 133: Inorder successor in a BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is a left child. O(h) time.
package main

import "fmt"

type Node struct {
	val                  int
	left, right, parent *Node
}

func successor(node *Node) *Node {
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
	p := node.parent
	for p != nil && node == p.right {
		node = p
		p = p.parent
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
	root := &Node{val: 10}
	root.left = attach(root, &Node{val: 5})
	root.right = attach(root, &Node{val: 30})
	n22 := &Node{val: 22}
	n35 := &Node{val: 35}
	root.right.left = attach(root.right, n22)
	root.right.right = attach(root.right, n35)

	s := successor(n22)
	if s != nil {
		fmt.Println(s.val) // 30
	} else {
		fmt.Println("null")
	}
}
