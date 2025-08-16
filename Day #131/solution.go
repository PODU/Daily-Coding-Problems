// Day 131: Deep clone a linked list with next + random pointers.
// Interleaving trick (weave copies, set randoms, unweave). O(n) time, O(1) extra space.
package main

import "fmt"

type Node struct {
	val          int
	next, random *Node
}

func clone(head *Node) *Node {
	if head == nil {
		return nil
	}
	for c := head; c != nil; c = c.next.next {
		cp := &Node{val: c.val, next: c.next}
		c.next = cp
	}
	for c := head; c != nil; c = c.next.next {
		if c.random != nil {
			c.next.random = c.random.next
		}
	}
	newHead := head.next
	for c := head; c != nil; c = c.next {
		cp := c.next
		c.next = cp.next
		if cp.next != nil {
			cp.next = cp.next.next
		}
	}
	return newHead
}

func main() {
	n := make([]*Node, 5)
	for v := 0; v < 5; v++ {
		n[v] = &Node{val: v + 1}
	}
	for i := 0; i < 4; i++ {
		n[i].next = n[i+1]
	}
	n[0].random = n[2]
	n[1].random = n[0]
	n[2].random = n[4]
	n[3].random = n[1]
	n[4].random = n[4]

	copy := clone(n[0])
	separate := true
	o, c := n[0], copy
	for c != nil {
		if c == o {
			separate = false
		}
		rv := 0
		if c.random != nil {
			rv = c.random.val
		}
		fmt.Printf("node %d -> random %d\n", c.val, rv)
		o = o.next
		c = c.next
	}
	fmt.Printf("deep copy verified: %t\n", separate)
}
