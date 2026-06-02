// Approach: Symmetric k-ary tree via isMirror recursion comparing children mirror-wise.
// Time O(n), Space O(h) recursion.
package main

import "fmt"

type Node struct {
	val      int
	children []*Node
}

func isMirror(a, b *Node) bool {
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
	k := len(a.children)
	for i := 0; i < k; i++ {
		if !isMirror(a.children[i], b.children[k-1-i]) {
			return false
		}
	}
	return true
}

func isSymmetric(root *Node) bool {
	if root == nil {
		return true
	}
	return isMirror(root, root)
}

func main() {
	root := &Node{val: 4}
	c1 := &Node{val: 3, children: []*Node{{val: 9}}}
	c2 := &Node{val: 5}
	c3 := &Node{val: 3, children: []*Node{{val: 9}}}
	root.children = []*Node{c1, c2, c3}
	if isSymmetric(root) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
