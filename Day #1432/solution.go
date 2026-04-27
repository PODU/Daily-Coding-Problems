// Day 1432: Deep clone a linked list with a random pointer.
// Approach: interleave cloned nodes, wire randoms, then split. Time: O(n), Space: O(1) extra.
package main

import "fmt"

type Node struct {
	val    int
	next   *Node
	random *Node
}

func cloneList(head *Node) *Node {
	if head == nil {
		return nil
	}
	for cur := head; cur != nil; cur = cur.next.next {
		copy := &Node{val: cur.val, next: cur.next}
		cur.next = copy
	}
	for cur := head; cur != nil; cur = cur.next.next {
		if cur.random != nil {
			cur.next.random = cur.random.next
		}
	}
	newHead := head.next
	for cur := head; cur != nil; cur = cur.next {
		copy := cur.next
		cur.next = copy.next
		if copy.next != nil {
			copy.next = copy.next.next
		}
	}
	return newHead
}

func main() {
	a := &Node{val: 1}
	b := &Node{val: 2}
	c := &Node{val: 3}
	a.next, b.next = b, c
	a.random, b.random, c.random = c, a, c

	cloned := cloneList(a)
	ok := true
	p, q := a, cloned
	for p != nil {
		if q == p {
			ok = false
		}
		if q.val != p.val {
			ok = false
		}
		if (p.random == nil) != (q.random == nil) {
			ok = false
		}
		if p.random != nil && q.random.val != p.random.val {
			ok = false
		}
		p, q = p.next, q.next
	}
	if ok {
		fmt.Println("Clone verified: values and random targets match, nodes distinct")
	} else {
		fmt.Println("Clone FAILED")
	}
}
