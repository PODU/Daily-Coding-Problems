// Sort a linked list in O(n log n) time, O(1) extra space.
// Bottom-up (iterative) merge sort on the list; no recursion stack -> constant space.
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

func split(head *Node, size int) *Node {
	for i := 1; head != nil && i < size; i++ {
		head = head.next
	}
	if head == nil {
		return nil
	}
	rest := head.next
	head.next = nil
	return rest
}

func merge(a, b, tail *Node) *Node {
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
	return tail
}

func sortList(head *Node) *Node {
	if head == nil || head.next == nil {
		return head
	}
	n := 0
	for p := head; p != nil; p = p.next {
		n++
	}
	dummy := &Node{next: head}
	for size := 1; size < n; size *= 2 {
		cur := dummy.next
		tail := dummy
		for cur != nil {
			left := cur
			right := split(left, size)
			cur = split(right, size)
			tail = merge(left, right, tail)
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
