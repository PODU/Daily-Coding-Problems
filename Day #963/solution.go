// Day 963: Find intersecting node of two singly linked lists.
// Approach: two pointers swap heads; meet at intersection. Time O(M+N), Space O(1).
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func getIntersection(a, b *Node) *Node {
	if a == nil || b == nil {
		return nil
	}
	p, q := a, b
	for p != q {
		if p == nil {
			p = b
		} else {
			p = p.next
		}
		if q == nil {
			q = a
		} else {
			q = q.next
		}
	}
	return p
}

func main() {
	n8 := &Node{val: 8}
	n8.next = &Node{val: 10}
	a := &Node{val: 3, next: &Node{val: 7, next: n8}}
	b := &Node{val: 99, next: &Node{val: 1, next: n8}}

	fmt.Printf("the node with value %d\n", getIntersection(a, b).val)
}
