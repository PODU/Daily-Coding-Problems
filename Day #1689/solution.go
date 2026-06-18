// Sort a singly linked list via bottom-up (iterative) merge sort.
// O(n log n) time, O(1) extra space (no recursion).
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

func length(h *Node) int {
	n := 0
	for h != nil {
		n++
		h = h.next
	}
	return n
}

// split cuts the list to its first n nodes and returns the remainder.
func split(head *Node, n int) *Node {
	for i := 1; head != nil && i < n; i++ {
		head = head.next
	}
	if head == nil {
		return nil
	}
	second := head.next
	head.next = nil
	return second
}

func merge(a, b *Node) *Node {
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
	return dummy.next
}

func sortList(head *Node) *Node {
	if head == nil {
		return nil
	}
	n := length(head)
	dummy := &Node{next: head}
	for size := 1; size < n; size <<= 1 {
		prev := dummy
		cur := dummy.next
		for cur != nil {
			left := cur
			right := split(left, size)
			cur = split(right, size)
			prev.next = merge(left, right)
			for prev.next != nil {
				prev = prev.next
			}
		}
	}
	return dummy.next
}

func main() {
	vals := []int{4, 1, -3, 99}
	var head, tail *Node
	for _, v := range vals {
		nd := &Node{val: v}
		if head == nil {
			head, tail = nd, nd
		} else {
			tail.next = nd
			tail = nd
		}
	}
	head = sortList(head)
	var parts []string
	for p := head; p != nil; p = p.next {
		parts = append(parts, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
