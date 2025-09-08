// Symmetric k-ary tree: a tree is symmetric iff left subtree mirrors right subtree.
// Recursively compare children in mirrored order. Time: O(N), Space: O(height).
package main

import "fmt"

type Node struct {
	val      int
	children []*Node
}

func mirror(a, b *Node) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	if a.val != b.val || len(a.children) != len(b.children) {
		return false
	}
	n := len(a.children)
	for i := 0; i < n; i++ {
		if !mirror(a.children[i], b.children[n-1-i]) {
			return false
		}
	}
	return true
}

func isSymmetric(root *Node) bool {
	if root == nil {
		return true
	}
	n := len(root.children)
	for i := 0; i < n/2; i++ {
		if !mirror(root.children[i], root.children[n-1-i]) {
			return false
		}
	}
	return true
}

func main() {
	root := &Node{4, []*Node{
		{3, []*Node{{9, nil}}},
		{5, nil},
		{3, []*Node{{9, nil}}},
	}}
	fmt.Println(isSymmetric(root)) // true
}
