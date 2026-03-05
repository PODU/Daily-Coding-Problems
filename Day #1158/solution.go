// Bottom-up iterative merge sort on a singly linked list. O(n log n) time, O(1) auxiliary space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

// split takes n nodes from head, cuts, returns remainder.
func split(head *ListNode, n int) *ListNode {
	for i := 1; head != nil && i < n; i++ {
		head = head.Next
	}
	if head == nil {
		return nil
	}
	second := head.Next
	head.Next = nil
	return second
}

// merge sorted lists a, b onto tail; returns new tail.
func merge(a, b, tail *ListNode) *ListNode {
	cur := tail
	for a != nil && b != nil {
		if a.Val <= b.Val {
			cur.Next = a
			a = a.Next
		} else {
			cur.Next = b
			b = b.Next
		}
		cur = cur.Next
	}
	if a != nil {
		cur.Next = a
	} else {
		cur.Next = b
	}
	for cur.Next != nil {
		cur = cur.Next
	}
	return cur
}

func sortList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	n := 0
	for p := head; p != nil; p = p.Next {
		n++
	}

	dummy := &ListNode{}
	dummy.Next = head
	for size := 1; size < n; size <<= 1 {
		cur := dummy.Next
		tail := dummy
		for cur != nil {
			left := cur
			right := split(left, size)
			cur = split(right, size)
			tail = merge(left, right, tail)
		}
	}
	return dummy.Next
}

func main() {
	vals := []int{4, 1, -3, 99}
	var head, tail *ListNode
	for _, v := range vals {
		node := &ListNode{Val: v}
		if head == nil {
			head, tail = node, node
		} else {
			tail.Next = node
			tail = node
		}
	}

	head = sortList(head)

	var parts []string
	for p := head; p != nil; p = p.Next {
		parts = append(parts, strconv.Itoa(p.Val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
