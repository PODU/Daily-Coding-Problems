// K-ary tree symmetry: recursively mirror-match children list. O(n) time/space.
package main

import "fmt"

type Node struct {
	val      int
	children []*Node
}

func isMirror(L, R *Node) bool {
	if L == nil && R == nil { return true }
	if L == nil || R == nil || L.val != R.val { return false }
	n := len(L.children)
	if len(R.children) != n { return false }
	for i := 0; i < n; i++ {
		if !isMirror(L.children[i], R.children[n-1-i]) { return false }
	}
	return true
}

func isSymmetric(root *Node) bool {
	if root == nil { return true }
	n := len(root.children)
	for i := 0; i < n/2; i++ {
		if !isMirror(root.children[i], root.children[n-1-i]) { return false }
	}
	return true
}

func main() {
	// Symmetric: 4 -> [3,5,3], first 3 -> [9], last 3 -> [9]
	root := &Node{4, []*Node{
		{3, []*Node{{9, nil}}},
		{5, nil},
		{3, []*Node{{9, nil}}},
	}}
	fmt.Println("Symmetric:", isSymmetric(root))

	// Asymmetric: 1 -> [2,3]
	r2 := &Node{1, []*Node{{2, nil}, {3, nil}}}
	fmt.Println("Symmetric:", isSymmetric(r2))
}
