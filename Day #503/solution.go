// Day 503: Sort a singly linked list using bottom-up (iterative) merge sort.
// Time O(n log n), Space O(1) auxiliary (no recursion).
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

func listLength(head *Node) int {
	n := 0
	for ; head != nil; head = head.next {
		n++
	}
	return n
}

// split off `size` nodes from head; cut there and return the rest.
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

// merge two sorted lists after tail; return new tail.
func mergeLists(a, b, tail *Node) *Node {
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
	n := listLength(head)
	dummy := &Node{}
	dummy.next = head
	for size := 1; size < n; size *= 2 {
		tail := dummy
		cur := dummy.next
		for cur != nil {
			left := cur
			right := split(left, size)
			cur = split(right, size)
			tail = mergeLists(left, right, tail)
		}
	}
	return dummy.next
}

func printList(head *Node) {
	var parts []string
	for ; head != nil; head = head.next {
		parts = append(parts, strconv.Itoa(head.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	vals := []int{4, 1, -3, 99}
	dummy := &Node{}
	tail := dummy
	for _, v := range vals {
		tail.next = &Node{val: v}
		tail = tail.next
	}
	sorted := sortList(dummy.next)
	printList(sorted) // -3 -> 1 -> 4 -> 99
}
