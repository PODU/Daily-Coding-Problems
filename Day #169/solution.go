// Bottom-up (iterative) merge sort on a singly linked list. O(n log n) time, O(1) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func merge(a, b *Node) (*Node, *Node) {
	dummy := &Node{}
	tail := dummy
	for a != nil && b != nil {
		if a.val <= b.val {
			tail.next = a
			a = a.next
		} else {
			tail.next = b
			b = b.next
		}
		tail = tail.next
	}
	if a != nil {
		tail.next = a
	} else {
		tail.next = b
	}
	for tail.next != nil {
		tail = tail.next
	}
	return dummy.next, tail
}

func length(h *Node) int {
	n := 0
	for h != nil {
		n++
		h = h.next
	}
	return n
}

func split(head *Node, n int) *Node {
	for i := 1; head != nil && i < n; i++ {
		head = head.next
	}
	if head == nil {
		return nil
	}
	rest := head.next
	head.next = nil
	return rest
}

func sortList(head *Node) *Node {
	n := length(head)
	dummy := &Node{next: head}
	for size := 1; size < n; size <<= 1 {
		cur := dummy.next
		tail := dummy
		for cur != nil {
			left := cur
			right := split(left, size)
			cur = split(right, size)
			mHead, mTail := merge(left, right)
			tail.next = mHead
			tail = mTail
		}
	}
	return dummy.next
}

func main() {
	vals := []int{4, 1, -3, 99}
	dummy := &Node{}
	t := dummy
	for _, v := range vals {
		t.next = &Node{val: v}
		t = t.next
	}
	head := sortList(dummy.next)
	var parts []string
	for p := head; p != nil; p = p.next {
		parts = append(parts, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
