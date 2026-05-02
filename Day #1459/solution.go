// Invert binary tree by swapping children recursively; print level-order (BFS).
// Time O(n), Space O(n).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         string
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
	a := &Node{val: "a"}
	b := &Node{val: "b"}
	c := &Node{val: "c"}
	d := &Node{val: "d"}
	e := &Node{val: "e"}
	f := &Node{val: "f"}
	a.left, a.right = b, c
	b.left, b.right = d, e
	c.left = f

	invert(a)

	var out []string
	queue := []*Node{a}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		if n == nil {
			continue
		}
		out = append(out, n.val)
		queue = append(queue, n.left, n.right)
	}
	fmt.Println(strings.Join(out, " "))
}
