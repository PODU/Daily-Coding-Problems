// Merge two binary trees by summing overlapping nodes; recurse and reuse the
// non-null subtree when only one side exists. Time O(n), Space O(h).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

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

	var out []string
	queue := []*Node{m}
	for len(queue) > 0 {
		cur := queue[0]
		queue = queue[1:]
		if cur != nil {
			out = append(out, strconv.Itoa(cur.val))
			queue = append(queue, cur.left, cur.right)
		} else {
			out = append(out, "null")
		}
	}
	for len(out) > 0 && out[len(out)-1] == "null" {
		out = out[:len(out)-1]
	}

	fmt.Println(strings.Join(out, " "))
}
