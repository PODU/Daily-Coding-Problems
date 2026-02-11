// Inorder successor in a BST using parent pointers.
// If node has right subtree -> leftmost of right subtree; else walk up parents
// until coming from a left child. Time O(h), Space O(1).
package main

import "fmt"

type Node struct {
	val    int
	left   *Node
	right  *Node
	parent *Node
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
	p := node.parent
	for p != nil && p.right == cur {
		cur = p
		p = p.parent
	}
	return p
}

func main() {
	root := &Node{val: 10}
	n5 := &Node{val: 5}
	n30 := &Node{val: 30}
	n22 := &Node{val: 22}
	n35 := &Node{val: 35}
	root.left, n5.parent = n5, root
	root.right, n30.parent = n30, root
	n30.left, n22.parent = n22, n30
	n30.right, n35.parent = n35, n30

	succ := inorderSuccessor(n22)
	if succ != nil {
		fmt.Println(succ.val)
	} else {
		fmt.Println("null")
	}
}
