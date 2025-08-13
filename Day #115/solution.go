// Day 115: Subtree check via recursive structural match. O(|s|*|t|) worst case.
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func same(a, b *Node) bool {
	if a == nil && b == nil {
		return true
	}
	if a == nil || b == nil || a.Val != b.Val {
		return false
	}
	return same(a.Left, b.Left) && same(a.Right, b.Right)
}

func isSubtree(s, t *Node) bool {
	if t == nil {
		return true
	}
	if s == nil {
		return false
	}
	if same(s, t) {
		return true
	}
	return isSubtree(s.Left, t) || isSubtree(s.Right, t)
}

func main() {
	s := &Node{Val: 3,
		Left:  &Node{Val: 4, Left: &Node{Val: 1}, Right: &Node{Val: 2}},
		Right: &Node{Val: 5}}
	t := &Node{Val: 4, Left: &Node{Val: 1}, Right: &Node{Val: 2}}
	u := &Node{Val: 4, Left: &Node{Val: 0}}
	fmt.Println(isSubtree(s, t)) // true
	fmt.Println(isSubtree(s, u)) // false
}
