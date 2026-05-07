// Day 1485: Determine whether a k-ary tree is symmetric about its root.
// Subtrees mirror iff values match and child i mirrors child (k-1-i).
// Time O(N), Space O(H).
package main

import "fmt"

type Node struct {
	val      int
	children []*Node
}

func isMirror(a, b *Node) bool {
	if a.val != b.val || len(a.children) != len(b.children) {
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
	return root == nil || isMirror(root, root)
}

func main() {
	tree := &Node{4, []*Node{
		{3, []*Node{{9, nil}}},
		{5, nil},
		{3, []*Node{{9, nil}}},
	}}
	fmt.Println(isSymmetric(tree)) // true
}
