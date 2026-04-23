// Day 1411: Check if tree t is a subtree of tree s.
// Approach: recursive DFS — for each node of s try exact-match with t. O(|s|*|t|) time, O(h) space.
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
	s := &Node{3, &Node{4, &Node{1, nil, nil}, &Node{2, nil, nil}}, &Node{5, nil, nil}}
	t := &Node{4, &Node{1, nil, nil}, &Node{2, nil, nil}}
	if isSubtree(s, t) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
