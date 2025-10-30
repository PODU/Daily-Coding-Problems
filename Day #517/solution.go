// Intersection: two pointers switch lists after end; meet at the join. O(M+N) time, O(1) space.
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
	shared := &Node{8, &Node{10, nil}}
	A := &Node{3, &Node{7, shared}}
	B := &Node{99, &Node{1, shared}}
	fmt.Println("The node with value", getIntersection(A, B).val)
}
