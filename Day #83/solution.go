// Day 83: Invert (mirror) a binary tree by swapping children recursively.
// Time O(n), Space O(h).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	Val         byte
	Left, Right *Node
}

func invert(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.Left, root.Right = root.Right, root.Left
	invert(root.Left)
	invert(root.Right)
	return root
}

func levelOrder(root *Node) string {
	out := []string{}
	q := []*Node{}
	if root != nil {
		q = append(q, root)
	}
	for len(q) > 0 {
		n := q[0]
		q = q[1:]
		out = append(out, string(n.Val))
		if n.Left != nil {
			q = append(q, n.Left)
		}
		if n.Right != nil {
			q = append(q, n.Right)
		}
	}
	return strings.Join(out, " ")
}

func main() {
	a := &Node{Val: 'a',
		Left:  &Node{Val: 'b', Left: &Node{Val: 'd'}, Right: &Node{Val: 'e'}},
		Right: &Node{Val: 'c', Left: &Node{Val: 'f'}}}
	fmt.Println("before:", levelOrder(a)) // a b c d e f
	invert(a)
	fmt.Println("after: ", levelOrder(a)) // a c b f e d
}
