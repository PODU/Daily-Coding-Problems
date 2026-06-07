// Day 1625: Inorder successor in BST using parent pointers.
// If right subtree exists, leftmost of it; else climb until node is left child. O(h).
package main

import "fmt"

type Node struct {
	val                   int
	left, right, parent *Node
}

func inorderSuccessor(node *Node) *Node {
	if node == nil {
		return nil
	}
	if node.right != nil {
		cur := node.right
		for cur.left != nil {
			cur = cur.left
		}
		return cur
	}
	cur := node
	for cur.parent != nil && cur.parent.right == cur {
		cur = cur.parent
	}
	return cur.parent
}

func insert(root *Node, v int) *Node {
	if root == nil {
		return &Node{val: v}
	}
	cur := root
	for {
		if v < cur.val {
			if cur.left == nil {
				cur.left = &Node{val: v, parent: cur}
				return root
			}
			cur = cur.left
		} else {
			if cur.right == nil {
				cur.right = &Node{val: v, parent: cur}
				return root
			}
			cur = cur.right
		}
	}
}

func find(root *Node, v int) *Node {
	for root != nil && root.val != v {
		if v < root.val {
			root = root.left
		} else {
			root = root.right
		}
	}
	return root
}

func main() {
	var root *Node
	for _, v := range []int{10, 5, 30, 22, 35} {
		root = insert(root, v)
	}
	s := inorderSuccessor(find(root, 22))
	if s != nil {
		fmt.Printf("The inorder successor of 22 is %d.\n", s.val)
	} else {
		fmt.Println("The inorder successor of 22 is None.")
	}
}
