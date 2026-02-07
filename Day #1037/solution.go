// Sorted array -> height-balanced BST: recursively pick middle as root.
// Time: O(n), Space: O(log n) recursion.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
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

func printRotated(node *Node, depth int) {
	if node == nil {
		return
	}
	printRotated(node.right, depth+1)
	fmt.Println(strings.Repeat(" ", depth*4) + fmt.Sprintf("%d", node.val))
	printRotated(node.left, depth+1)
}

func inorder(node *Node, out *[]int) {
	if node == nil {
		return
	}
	inorder(node.left, out)
	*out = append(*out, node.val)
	inorder(node.right, out)
}

func main() {
	a := []int{-10, -3, 0, 5, 9}
	root := build(a, 0, len(a)-1)
	fmt.Println("Height-balanced BST (rotated 90 deg):")
	printRotated(root, 0)
	var io []int
	inorder(root, &io)
	parts := make([]string, len(io))
	for i, x := range io {
		parts[i] = fmt.Sprintf("%d", x)
	}
	fmt.Println("In-order: " + strings.Join(parts, " "))
}
