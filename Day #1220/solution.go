// Merge two binary trees recursively (sum overlaps, keep lone nodes). O(min(n1,n2)) time.
// Serialize merged tree as BFS level-order with trailing nulls trimmed.
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

func serialize(root *Node) string {
	var out []string
	q := []*Node{root}
	for i := 0; i < len(q); i++ {
		n := q[i]
		if n == nil {
			out = append(out, "null")
			continue
		}
		out = append(out, strconv.Itoa(n.val))
		q = append(q, n.left, n.right)
	}
	for len(out) > 0 && out[len(out)-1] == "null" {
		out = out[:len(out)-1]
	}
	return "[" + strings.Join(out, ", ") + "]"
}

func main() {
	t1 := &Node{val: 1,
		left:  &Node{val: 3, left: &Node{val: 5}},
		right: &Node{val: 2}}
	t2 := &Node{val: 2,
		left:  &Node{val: 1, right: &Node{val: 4}},
		right: &Node{val: 3, right: &Node{val: 7}}}
	fmt.Println(serialize(merge(t1, t2)))
}
