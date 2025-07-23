// Intersection of two linked lists: two-pointer switch trick.
// Time O(M+N), Space O(1).
package main

import "fmt"

type Node struct {
	val  int
	next *Node
}

func getIntersection(a, b *Node) *Node {
	pa, pb := a, b
	for pa != pb {
		if pa == nil {
			pa = b
		} else {
			pa = pa.next
		}
		if pb == nil {
			pb = a
		} else {
			pb = pb.next
		}
	}
	return pa
}

func main() {
	shared := &Node{val: 8}
	shared.next = &Node{val: 10}

	a := &Node{val: 3}
	a.next = &Node{val: 7}
	a.next.next = shared

	b := &Node{val: 99}
	b.next = &Node{val: 1}
	b.next.next = shared

	fmt.Println(getIntersection(a, b).val)
}
