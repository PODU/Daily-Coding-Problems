// Day 1418: inorder successor of a BST node using parent pointers.
// Approach: if right subtree exists, leftmost of it; else climb until node is a left child. O(h).
package main

import "fmt"

type Node struct {
	Val                   int
	Left, Right, Parent   *Node
}

func successor(node *Node) *Node {
	if node.Right != nil {
		cur := node.Right
		for cur.Left != nil {
			cur = cur.Left
		}
		return cur
	}
	cur := node
	for cur.Parent != nil && cur == cur.Parent.Right {
		cur = cur.Parent
	}
	return cur.Parent
}

func attach(parent, child *Node) *Node {
	if child != nil {
		child.Parent = parent
	}
	return child
}

func main() {
	root := &Node{Val: 10}
	root.Left = attach(root, &Node{Val: 5})
	root.Right = attach(root, &Node{Val: 30})
	n22 := &Node{Val: 22}
	root.Right.Left = attach(root.Right, n22)
	root.Right.Right = attach(root.Right, &Node{Val: 35})

	s := successor(n22)
	if s != nil {
		fmt.Println(s.Val) // 30
	} else {
		fmt.Println("null")
	}
}
