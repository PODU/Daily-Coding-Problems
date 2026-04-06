// Rearrange linked list values to low->high->low->high. One pass swapping
// adjacent values to enforce the alternating relation. Time O(n), Space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func zigzag(head *Node) {
	low := true // current pair should satisfy a <= b
	for c := head; c != nil && c.next != nil; c, low = c.next, !low {
		if (low && c.val > c.next.val) || (!low && c.val < c.next.val) {
			c.val, c.next.val = c.next.val, c.val
		}
	}
}

func build(vals []int) *Node {
	var head, tail *Node
	for _, v := range vals {
		n := &Node{val: v}
		if head == nil {
			head, tail = n, n
		} else {
			tail.next = n
			tail = n
		}
	}
	return head
}

func toStr(head *Node) string {
	var parts []string
	for c := head; c != nil; c = c.next {
		parts = append(parts, fmt.Sprintf("%d", c.val))
	}
	return strings.Join(parts, " -> ")
}

func main() {
	h := build([]int{1, 2, 3, 4, 5})
	zigzag(h)
	fmt.Println(toStr(h)) // 1 -> 3 -> 2 -> 5 -> 4
}
