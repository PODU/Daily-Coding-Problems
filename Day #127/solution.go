// Day 127: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. O(max(m,n)) time, O(1) extra space.
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

func addLists(a, b *Node) *Node {
	dummy := &Node{}
	tail := dummy
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
		tail.next = &Node{val: s % 10}
		tail = tail.next
	}
	return dummy.next
}

func build(d []int) *Node {
	dummy := &Node{}
	t := dummy
	for _, v := range d {
		t.next = &Node{val: v}
		t = t.next
	}
	return dummy.next
}

func toStr(n *Node) string {
	var parts []string
	for n != nil {
		parts = append(parts, strconv.Itoa(n.val))
		n = n.next
	}
	return strings.Join(parts, " -> ")
}

func main() {
	a := build([]int{9, 9}) // 99
	b := build([]int{5, 2}) // 25
	fmt.Println(toStr(addLists(a, b)))
}
