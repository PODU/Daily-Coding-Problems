// Day 752: Level-order (BFS) traversal of a binary tree.
// Time: O(n), Space: O(w) where w is the max width of the tree.
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func levelOrder(root *Node) []int {
	out := []int{}
	if root == nil {
		return out
	}
	queue := []*Node{root}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		out = append(out, n.val)
		if n.left != nil {
			queue = append(queue, n.left)
		}
		if n.right != nil {
			queue = append(queue, n.right)
		}
	}
	return out
}

func main() {
	root := &Node{val: 1,
		left:  &Node{val: 2},
		right: &Node{val: 3, left: &Node{val: 4}, right: &Node{val: 5}}}

	res := levelOrder(root)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Println(strings.Join(parts, ", "))
}
