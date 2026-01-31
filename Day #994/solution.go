// Day 994: Print binary tree nodes level by level (BFS).
// Use a queue; dequeue a node, emit it, enqueue its children. O(n) time/space.
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

func levelOrder(root *Node) []int {
	var out []int
	var q []*Node
	if root != nil {
		q = append(q, root)
	}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		out = append(out, n.val)
		if n.left != nil {
			q = append(q, n.left)
		}
		if n.right != nil {
			q = append(q, n.right)
		}
	}
	return out
}

func main() {
	root := &Node{1, &Node{2, nil, nil}, &Node{3, &Node{4, nil, nil}, &Node{5, nil, nil}}}
	vals := levelOrder(root)
	parts := make([]string, len(vals))
	for i, v := range vals {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println(strings.Join(parts, ", ")) // 1, 2, 3, 4, 5
}
