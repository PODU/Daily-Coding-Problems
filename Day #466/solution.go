// Symmetric k-ary tree: recursively compare children of two subtrees in mirror order.
// Time: O(n), Space: O(h) recursion.
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
	if a.val != b.val {
		return false
	}
	if len(a.children) != len(b.children) {
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
	return mirror(root, root)
}

func main() {
	root := &Node{val: 4}
	l3 := &Node{val: 3}
	m5 := &Node{val: 5}
	r3 := &Node{val: 3}
	root.children = []*Node{l3, m5, r3}
	l3.children = []*Node{{val: 9}}
	r3.children = []*Node{{val: 9}}
	if isSymmetric(root) {
		fmt.Println("True")
	} else {
		fmt.Println("False")
	}
}
