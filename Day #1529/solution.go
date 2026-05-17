// Prune binary tree: remove subtrees containing only 0s (no 1 descendant).
// Post-order recursion. O(n) time, O(h) recursion stack.
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

func prune(root *Node) *Node {
	if root == nil {
		return nil
	}
	root.Left = prune(root.Left)
	root.Right = prune(root.Right)
	if root.Val == 0 && root.Left == nil && root.Right == nil {
		return nil
	}
	return root
}

func levelOrder(root *Node) string {
	queue := []*Node{root}
	var out []string
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		if n == nil {
			out = append(out, "null")
			continue
		}
		out = append(out, strconv.Itoa(n.Val))
		queue = append(queue, n.Left, n.Right)
	}
	for len(out) > 0 && out[len(out)-1] == "null" {
		out = out[:len(out)-1]
	}
	return "[" + strings.Join(out, ", ") + "]"
}

func main() {
	root := &Node{Val: 0,
		Left: &Node{Val: 1},
		Right: &Node{Val: 0,
			Left:  &Node{Val: 1, Left: &Node{Val: 0}, Right: &Node{Val: 0}},
			Right: &Node{Val: 0}}}
	fmt.Println(levelOrder(prune(root))) // [0, 1, 0, null, null, 1]
}
