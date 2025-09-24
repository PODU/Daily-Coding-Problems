// Merge two binary trees recursively (sum overlaps, keep lone nodes); print merged tree level-order skipping nulls.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	val   int
	left  *Node
	right *Node
}

func merge(a, b *Node) *Node {
	if a == nil {
		return b
	}
	if b == nil {
		return a
	}
	n := &Node{val: a.val + b.val}
	n.left = merge(a.left, b.left)
	n.right = merge(a.right, b.right)
	return n
}

func main() {
	t1 := &Node{val: 1}
	t1.left = &Node{val: 3}
	t1.right = &Node{val: 2}
	t1.left.left = &Node{val: 5}

	t2 := &Node{val: 2}
	t2.left = &Node{val: 1}
	t2.right = &Node{val: 3}
	t2.left.right = &Node{val: 4}
	t2.right.right = &Node{val: 7}

	m := merge(t1, t2)

	level := []*Node{m}
	for len(level) > 0 {
		var parts []string
		var next []*Node
		for _, c := range level {
			parts = append(parts, strconv.Itoa(c.val))
			if c.left != nil {
				next = append(next, c.left)
			}
			if c.right != nil {
				next = append(next, c.right)
			}
		}
		fmt.Println(strings.Join(parts, " "))
		level = next
	}
}
