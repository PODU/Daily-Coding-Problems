// Second largest in BST via parent-walk: find largest; if it has a left subtree,
// answer = max of that subtree, else answer = parent of largest. Time O(h), Space O(1).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func insert(root *Node, v int) *Node {
	if root == nil {
		return &Node{val: v}
	}
	if v < root.val {
		root.left = insert(root.left, v)
	} else {
		root.right = insert(root.right, v)
	}
	return root
}

func maxNode(n *Node) int {
	for n.right != nil {
		n = n.right
	}
	return n.val
}

func secondLargest(root *Node) int {
	cur := root
	var parent *Node
	for cur.right != nil {
		parent = cur
		cur = cur.right
	}
	if cur.left != nil {
		return maxNode(cur.left)
	}
	return parent.val
}

func main() {
	var root *Node
	for _, v := range []int{5, 3, 8, 2, 4, 7, 9} {
		root = insert(root, v)
	}
	fmt.Println(secondLargest(root))
}
