// Min root-to-leaf path sum via recursive DFS; leaf returns its val, internal node adds min of existing children.
// Reconstruct path by tracking the chosen child. Time O(n), space O(h).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	Val         int
	Left, Right *Node
}

// minPath returns the min path sum from node to a leaf and the path of values.
func minPath(node *Node) (int, []int) {
	if node.Left == nil && node.Right == nil {
		return node.Val, []int{node.Val}
	}
	bestSum := 0
	var bestPath []int
	first := true
	for _, child := range []*Node{node.Left, node.Right} {
		if child == nil {
			continue
		}
		s, p := minPath(child)
		if first || s < bestSum {
			bestSum, bestPath = s, p
			first = false
		}
	}
	path := append([]int{node.Val}, bestPath...)
	return node.Val + bestSum, path
}

func main() {
	root := &Node{Val: 10}
	root.Left = &Node{Val: 5}
	root.Right = &Node{Val: 5}
	root.Left.Right = &Node{Val: 2}
	root.Right.Right = &Node{Val: 1}
	root.Right.Right.Left = &Node{Val: -1}

	sum, path := minPath(root)

	parts := make([]string, len(path))
	for i, v := range path {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Printf("The minimum path is [%s], which has sum %d.\n", strings.Join(parts, ", "), sum)
}
