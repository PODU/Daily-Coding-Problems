// Day 107: Level-order (BFS) traversal of a binary tree. O(n) time, O(n) space.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func levelOrder(root *Node) []int {
	out := []int{}
	if root == nil {
		return out
	}
	q := []*Node{root}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		out = append(out, n.Val)
		if n.Left != nil {
			q = append(q, n.Left)
		}
		if n.Right != nil {
			q = append(q, n.Right)
		}
	}
	return out
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2},
		Right: &Node{Val: 3, Left: &Node{Val: 4}, Right: &Node{Val: 5}}}
	parts := []string{}
	for _, v := range levelOrder(root) {
		parts = append(parts, strconv.Itoa(v))
	}
	fmt.Println(strings.Join(parts, ", "))
}
