// Second largest in BST: walk right to largest; second largest is parent of
// largest (if no left subtree) else max of largest's left subtree. O(h) time, O(1) space.
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

func secondLargest(root *Node) int {
	var parent *Node
	cur := root
	for cur.right != nil {
		parent = cur
		cur = cur.right
	}
	if cur.left != nil {
		cur = cur.left
		for cur.right != nil {
			cur = cur.right
		}
		return cur.val
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
