// Day 422: Merge two binary trees recursively (value = sum), O(n) time, O(h) space.
// Then print merged tree by level-order traversal (skipping null children).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func merge(a, b *Node) *Node {
	if a == nil {
		return b
	}
	if b == nil {
		return a
	}
	return &Node{a.val + b.val, merge(a.left, b.left), merge(a.right, b.right)}
}

func main() {
	t1 := &Node{val: 1, left: &Node{val: 3, left: &Node{val: 5}}, right: &Node{val: 2}}
	t2 := &Node{val: 2, left: &Node{val: 1, right: &Node{val: 4}}, right: &Node{val: 3, right: &Node{val: 7}}}

	m := merge(t1, t2)

	q := []*Node{m}
	out := []int{}
	for len(q) > 0 {
		c := q[0]
		q = q[1:]
		out = append(out, c.val)
		if c.left != nil {
			q = append(q, c.left)
		}
		if c.right != nil {
			q = append(q, c.right)
		}
	}
	for i, x := range out {
		if i > 0 {
			fmt.Print(" ")
		}
		fmt.Print(x)
	}
	fmt.Println()
}
