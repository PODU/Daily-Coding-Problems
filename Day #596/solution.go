// Day 596: Invert a binary tree (mirror it).
// Approach: recursively swap left/right children. Time O(n), Space O(h).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         byte
	left, right *Node
}

func invert(root *Node) {
	if root == nil {
		return
	}
	root.left, root.right = root.right, root.left
	invert(root.left)
	invert(root.right)
}

func main() {
	a := &Node{val: 'a'}
	b := &Node{val: 'b'}
	c := &Node{val: 'c'}
	d := &Node{val: 'd'}
	e := &Node{val: 'e'}
	f := &Node{val: 'f'}
	a.left, a.right = b, c
	b.left, b.right = d, e
	c.left = f

	invert(a)

	q := []*Node{a}
	for len(q) > 0 {
		var next []*Node
		var line []string
		for _, n := range q {
			line = append(line, string(n.val))
			if n.left != nil {
				next = append(next, n.left)
			}
			if n.right != nil {
				next = append(next, n.right)
			}
		}
		fmt.Println(strings.Join(line, " "))
		q = next
	}
}
