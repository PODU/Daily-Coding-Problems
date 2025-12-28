// Add two numbers stored as reversed-digit linked lists via elementary addition
// with carry, walking both lists. Time O(max(m,n)), Space O(max(m,n)).
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

func build(digits []int) *ListNode {
	dummy := &ListNode{}
	cur := dummy
	for _, x := range digits {
		cur.next = &ListNode{val: x}
		cur = cur.next
	}
	return dummy.next
}

func addLists(a, b *ListNode) *ListNode {
	dummy := &ListNode{}
	cur := dummy
	carry := 0
	for a != nil || b != nil || carry != 0 {
		s := carry
		if a != nil {
			s += a.val
			a = a.next
		}
		if b != nil {
			s += b.val
			b = b.next
		}
		carry = s / 10
		cur.next = &ListNode{val: s % 10}
		cur = cur.next
	}
	return dummy.next
}

func toStr(n *ListNode) string {
	var parts []string
	for n != nil {
		parts = append(parts, strconv.Itoa(n.val))
		n = n.next
	}
	return strings.Join(parts, " -> ")
}

func main() {
	a := build([]int{9, 9})
	b := build([]int{5, 2})
	fmt.Println(toStr(addLists(a, b)))
}
