// Day 208: Partition a linked list around pivot k (stable).
// Build two lists (< k and >= k) in original order, then splice. Time: O(n), Space: O(1).
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

func partition(head *Node, k int) *Node {
	lessDummy, geDummy := &Node{}, &Node{}
	less, ge := lessDummy, geDummy
	for cur := head; cur != nil; cur = cur.next {
		if cur.val < k {
			less.next = cur
			less = cur
		} else {
			ge.next = cur
			ge = cur
		}
	}
	ge.next = nil
	less.next = geDummy.next
	return lessDummy.next
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

func toStr(h *Node) string {
	var parts []string
	for ; h != nil; h = h.next {
		parts = append(parts, strconv.Itoa(h.val))
	}
	return strings.Join(parts, " -> ")
}

func main() {
	fmt.Println(toStr(partition(build([]int{5, 1, 8, 0, 3}), 3))) // 1 -> 0 -> 5 -> 8 -> 3
}
