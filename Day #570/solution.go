// Subtree check: at each node of s, test sameTree(node, t).
// Time: O(|s|*|t|) worst case. Space: O(height). (Optimal O(|s|+|t|) via
// tree serialization + KMP substring search; recursive version implemented.)
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func sameTree(a, b *Node) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil {
		return false
	}
	return a.Val == b.Val && sameTree(a.Left, b.Left) && sameTree(a.Right, b.Right)
}

func isSubtree(s, t *Node) bool {
	if s == nil {
		return false
	}
	if sameTree(s, t) {
		return true
	}
	return isSubtree(s.Left, t) || isSubtree(s.Right, t)
}

func main() {
	// s:        3
	//          / \
	//         4   5
	//        / \
	//       1   2
	s := &Node{3,
		&Node{4, &Node{1, nil, nil}, &Node{2, nil, nil}},
		&Node{5, nil, nil}}
	t := &Node{4, &Node{1, nil, nil}, &Node{2, nil, nil}}
	t2 := &Node{4, &Node{1, nil, nil}, &Node{0, nil, nil}}

	fmt.Println(isSubtree(s, t))
	fmt.Println(isSubtree(s, t2))
}
