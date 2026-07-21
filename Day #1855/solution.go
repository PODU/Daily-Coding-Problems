// Day 1855: LCA in a binary tree with parent pointers.
// Two-pointer walk (like linked-list intersection): swap to the other node at root; meet at LCA. O(h) time, O(1) space.
package main

import "fmt"

type Node struct {
	val                 int
	left, right, parent *Node
}

func lca(p, q *Node) *Node {
	a, b := p, q
	for a != b {
		if a == nil {
			a = q
		} else {
			a = a.parent
		}
		if b == nil {
			b = p
		} else {
			b = b.parent
		}
	}
	return a
}

func attach(parent, child *Node) *Node {
	if child != nil {
		child.parent = parent
	}
	return child
}

func main() {
	n := make([]*Node, 9)
	for i := 1; i <= 8; i++ {
		n[i] = &Node{val: i}
	}
	n[1].left = attach(n[1], n[2]); n[1].right = attach(n[1], n[3])
	n[2].left = attach(n[2], n[4]); n[2].right = attach(n[2], n[5])
	n[3].right = attach(n[3], n[6])
	n[5].left = attach(n[5], n[7]); n[5].right = attach(n[5], n[8])

	fmt.Println(lca(n[7], n[8]).val) // 5
	fmt.Println(lca(n[7], n[6]).val) // 1
	fmt.Println(lca(n[4], n[8]).val) // 2
}
