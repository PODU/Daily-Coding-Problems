// Day 992: Second largest node in a BST.
// The largest is the rightmost node; the 2nd largest is its left subtree's max,
// or its parent if it has no left child. O(h) time, O(1) extra space.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func insert(root *Node, val int) *Node {
	if root == nil {
		return &Node{val: val}
	}
	if val < root.val {
		root.left = insert(root.left, val)
	} else {
		root.right = insert(root.right, val)
	}
	return root
}

func secondLargest(root *Node) int {
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
		return cur.val
	}
	return parent.val
}

func main() {
	var root *Node
	for _, v := range []int{5, 3, 8, 2, 4, 7, 9} {
		root = insert(root, v)
	}
	fmt.Println(secondLargest(root)) // 8
}
