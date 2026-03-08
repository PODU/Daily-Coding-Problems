// Day 1180: Swap every two adjacent nodes in a singly linked list.
// Iterative pointer rewiring with a dummy head. Time O(N), Space O(1).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type ListNode struct {
	val  int
	next *ListNode
}

func swapPairs(head *ListNode) *ListNode {
	dummy := &ListNode{next: head}
	prev := dummy
	for prev.next != nil && prev.next.next != nil {
		a := prev.next
		b := a.next
		a.next = b.next
		b.next = a
		prev.next = b
		prev = a
	}
	return dummy.next
}

func build(vals []int) *ListNode {
	dummy := &ListNode{}
	t := dummy
	for _, x := range vals {
		t.next = &ListNode{val: x}
		t = t.next
	}
	return dummy.next
}

func toStr(h *ListNode) string {
	var parts []string
	for ; h != nil; h = h.next {
		parts = append(parts, strconv.Itoa(h.val))
	}
	return strings.Join(parts, " -> ")
}

func main() {
	fmt.Println(toStr(swapPairs(build([]int{1, 2, 3, 4})))) // 2 -> 1 -> 4 -> 3
}
