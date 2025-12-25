// Day 800: Rearrange list values into low->high->low... (wiggle).
// One pass: at even idx ensure a<=next, at odd idx ensure a>=next; swap if not.
// Time O(N), Space O(1).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val  int
	next *Node
}

func wiggle(head *Node) {
	less := true
	for cur := head; cur != nil && cur.next != nil; cur = cur.next {
		var bad bool
		if less {
			bad = cur.val > cur.next.val
		} else {
			bad = cur.val < cur.next.val
		}
		if bad {
			cur.val, cur.next.val = cur.next.val, cur.val
		}
		less = !less
	}
}

func main() {
	head := &Node{val: 1}
	c := head
	for _, v := range []int{2, 3, 4, 5} {
		c.next = &Node{val: v}
		c = c.next
	}
	wiggle(head)
	var parts []string
	for p := head; p != nil; p = p.next {
		parts = append(parts, fmt.Sprintf("%d", p.val))
	}
	fmt.Println(strings.Join(parts, " -> ")) // 1 -> 3 -> 2 -> 5 -> 4
}
