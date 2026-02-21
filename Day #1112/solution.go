// Day 1112 - Minimum root-to-leaf path sum (return the path)
// Approach: DFS, track best leaf path by sum. Time: O(n), Space: O(h).
package main

import (
	"fmt"
	"math"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

var bestSum = math.MaxInt64
var bestPath []int

func dfs(node *Node, path []int, s int) {
	if node == nil {
		return
	}
	path = append(path, node.val)
	s += node.val
	if node.left == nil && node.right == nil {
		if s < bestSum {
			bestSum = s
			bestPath = append([]int(nil), path...)
		}
	} else {
		dfs(node.left, path, s)
		dfs(node.right, path, s)
	}
}

func main() {
	root := &Node{10,
		&Node{5, nil, &Node{2, nil, nil}},
		&Node{5, nil, &Node{1, &Node{-1, nil, nil}, nil}}}
	dfs(root, nil, 0)
	parts := make([]string, len(bestPath))
	for i, v := range bestPath {
		parts[i] = fmt.Sprintf("%d", v)
	}
	fmt.Printf("[%s], which has sum %d\n", strings.Join(parts, ", "), bestSum)
}
