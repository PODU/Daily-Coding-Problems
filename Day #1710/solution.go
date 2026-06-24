// Deep clone list w/ random ptr: interleave clones, wire randoms, unweave. O(n) time, O(1) extra.
package main

import "fmt"

type Node struct {
	val    int
	next   *Node
	random *Node
}

func copyRandomList(head *Node) *Node {
	if head == nil {
		return nil
	}
	for c := head; c != nil; c = c.next.next {
		cl := &Node{val: c.val, next: c.next}
		c.next = cl
	}
	for c := head; c != nil; c = c.next.next {
		if c.random != nil {
			c.next.random = c.random.next
		}
	}
	newHead := head.next
	for c := head; c != nil; c = c.next {
		cl := c.next
		c.next = cl.next
		if cl.next != nil {
			cl.next = cl.next.next
		}
	}
	return newHead
}

func main() {
	n1 := &Node{val: 1}
	n2 := &Node{val: 2}
	n3 := &Node{val: 3}
	n4 := &Node{val: 4}
	n1.next, n2.next, n3.next = n2, n3, n4
	n1.random = n3
	n2.random = n1
	n3.random = n3
	n4.random = n2

	for c := copyRandomList(n1); c != nil; c = c.next {
		fmt.Printf("node %d, random %d\n", c.val, c.random.val)
	}
}
