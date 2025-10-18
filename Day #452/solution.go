// Day 452: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(n,m)), Space O(max(n,m)).
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

func build(xs ...int) *Node {
	dummy := &Node{}
	t := dummy
	for _, x := range xs {
		t.next = &Node{val: x}
		t = t.next
	}
	return dummy.next
}

func show(n *Node) {
	var parts []string
	for n != nil {
		parts = append(parts, strconv.Itoa(n.val))
		n = n.next
	}
	fmt.Println(strings.Join(parts, " -> "))
}

func main() {
	show(addLists(build(9, 9), build(5, 2))) // 4 -> 2 -> 1
}
