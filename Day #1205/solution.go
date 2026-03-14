// Day 1205: Add two numbers stored as reversed-digit linked lists.
// Traverse both lists with a running carry. Time O(max(m,n)), Space O(max(m,n)).
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

func build(ds []int) *Node {
	dummy := &Node{}
	t := dummy
	for _, d := range ds {
		t.next = &Node{val: d}
		t = t.next
	}
	return dummy.next
}

func addLists(a, b *Node) *Node {
	dummy := &Node{}
	t := dummy
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
		t.next = &Node{val: s % 10}
		t = t.next
	}
	return dummy.next
}

func main() {
	s := addLists(build([]int{9, 9}), build([]int{5, 2}))
	var out []string
	for p := s; p != nil; p = p.next {
		out = append(out, strconv.Itoa(p.val))
	}
	fmt.Println(strings.Join(out, " -> ")) // 4 -> 2 -> 1
}
