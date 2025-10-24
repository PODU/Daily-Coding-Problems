// Day 484: Second largest node in a BST.
// O(h): walk right to largest; second largest is its parent, or max of largest's left subtree.
// Time O(h), Space O(1).
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

func secondLargest(root *Node) *Node {
	if root == nil || (root.left == nil && root.right == nil) {
		return nil
	}
	cur, parent := root, (*Node)(nil)
	for cur.right != nil {
		parent = cur
		cur = cur.right
	}
	if cur.left != nil {
		cur = cur.left
		for cur.right != nil {
			cur = cur.right
		}
		return cur
	}
	return parent
}

func main() {
	var root *Node
	for _, v := range []int{5, 3, 8, 2, 4, 7, 10} {
		root = insert(root, v)
	}
	fmt.Println(secondLargest(root).val) // 8
}
