// Largest BST subtree: bottom-up DFS returning {isBST, size, min, max}; combine children.
// Time O(n), Space O(h).
package main

import (
	"fmt"
	"math"
)

type Node struct {
	val         int
	left, right *Node
}

type info struct {
	isBST    bool
	size     int
	min, max int
}

var best int

func dfs(node *Node) info {
	if node == nil {
		return info{true, 0, math.MaxInt32, math.MinInt32}
	}
	l := dfs(node.left)
	r := dfs(node.right)
	if l.isBST && r.isBST && node.val > l.max && node.val < r.min {
		size := l.size + r.size + 1
		if size > best {
			best = size
		}
		return info{true, size, min(node.val, l.min), max(node.val, r.max)}
	}
	return info{false, 0, 0, 0}
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

func main() {
	root := &Node{val: 10}
	root.left = &Node{val: 5}
	root.right = &Node{val: 15}
	root.left.left = &Node{val: 1}
	root.left.right = &Node{val: 8}
	root.right.right = &Node{val: 7}
	dfs(root)
	fmt.Println(best)
}
