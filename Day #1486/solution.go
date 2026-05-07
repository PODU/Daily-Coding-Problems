// Day 1486: Partition a linked list around pivot k (stable).
// Approach: build two sublists (< k and >= k), then concatenate. O(n) time, O(1) extra space.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func partition(head *Node, k int) *Node {
	lessDummy := &Node{}
	geDummy := &Node{}
	less, ge := lessDummy, geDummy
	for cur := head; cur != nil; cur = cur.next {
		if cur.val < k {
			less.next = cur
			less = cur
		} else {
			ge.next = cur
			ge = cur
		}
	}
	ge.next = nil
	less.next = geDummy.next
	return lessDummy.next
}

func build(vals []int) *Node {
	var head, tail *Node
	for _, v := range vals {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	return head
}

func main() {
	head := partition(build([]int{5, 1, 8, 0, 3}), 3)
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, fmt.Sprintf("%d", c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
