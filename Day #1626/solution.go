// Day 1626: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(m,n)), Space O(max(m,n)).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func addLists(a, b *Node) *Node {
	dummy := &Node{}
	tail := dummy
	carry := 0
	for a != nil || b != nil || carry != 0 {
		sum := carry
		if a != nil {
			sum += a.val
			a = a.next
		}
		if b != nil {
			sum += b.val
			b = b.next
		}
		carry = sum / 10
		tail.next = &Node{val: sum % 10}
		tail = tail.next
	}
	return dummy.next
}

func build(vals []int) *Node {
	dummy := &Node{}
	t := dummy
	for _, x := range vals {
		t.next = &Node{val: x}
		t = t.next
	}
	return dummy.next
}

func main() {
	r := addLists(build([]int{9, 9}), build([]int{5, 2}))
	var parts []string
	for c := r; c != nil; c = c.next {
		parts = append(parts, fmt.Sprintf("%d", c.val))
	}
	fmt.Println(strings.Join(parts, " -> "))
}
