// Day 699: Rotate a singly linked list right by k places.
// Approach: close into a ring, then break it (len - k%len) nodes ahead.
// Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func rotateRight(head *Node, k int) *Node {
	if head == nil || head.next == nil {
		return head
	}
	length := 1
	tail := head
	for tail.next != nil {
		tail = tail.next
		length++
	}
	k %= length
	if k == 0 {
		return head
	}
	tail.next = head // ring
	steps := length - k
	newTail := head
	for i := 1; i < steps; i++ {
		newTail = newTail.next
	}
	newHead := newTail.next
	newTail.next = nil
	return newHead
}

func build(vals []int) *Node {
	d := &Node{}
	c := d
	for _, x := range vals {
		c.next = &Node{val: x}
		c = c.next
	}
	return d.next
}

func show(h *Node) {
	var parts []string
	for ; h != nil; h = h.next {
		parts = append(parts, fmt.Sprintf("%d", h.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	show(rotateRight(build([]int{7, 7, 3, 5}), 2))    // 3 -> 5 -> 7 -> 7
	show(rotateRight(build([]int{1, 2, 3, 4, 5}), 3)) // 3 -> 4 -> 5 -> 1 -> 2
}
