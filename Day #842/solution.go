// Day 842: invert (mirror) a binary tree by swapping children at every node.
// Recursive DFS. O(n) time, O(h) stack space.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	v    byte
	l, r *Node
}

func invert(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.l, root.r = root.r, root.l
	invert(root.l)
	invert(root.r)
	return root
}

func levelOrder(root *Node) string {
	if root == nil {
		return ""
	}
	var out []string
	q := []*Node{root}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		out = append(out, string(n.v))
		if n.l != nil {
			q = append(q, n.l)
		}
		if n.r != nil {
			q = append(q, n.r)
		}
	}
	return strings.Join(out, " ")
}

func main() {
	a := &Node{v: 'a'}
	b := &Node{v: 'b'}
	c := &Node{v: 'c'}
	d := &Node{v: 'd'}
	e := &Node{v: 'e'}
	f := &Node{v: 'f'}
	a.l, a.r = b, c
	b.l, b.r = d, e
	c.l = f
	invert(a)
	fmt.Println(levelOrder(a)) // a c b f e d
}
