// Day 1325: Sorted array -> height-balanced BST.
// Recursively pick the middle element as the root so both halves differ in height by <=1. O(n) time, O(log n) stack.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func build(a []int, lo, hi int) *Node {
	if lo > hi {
		return nil
	}
	mid := lo + (hi-lo)/2
	root := &Node{val: a[mid]}
	root.left = build(a, lo, mid-1)
	root.right = build(a, mid+1, hi)
	return root
}

func preorder(n *Node, out *[]int) {
	if n == nil {
		return
	}
	*out = append(*out, n.val)
	preorder(n.left, out)
	preorder(n.right, out)
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6, 7}
	root := build(a, 0, len(a)-1)
	var out []int
	preorder(root, &out)
	fmt.Println("preorder:", out) // preorder: [4 2 1 3 6 5 7]
}
