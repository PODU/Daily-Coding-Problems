// Day 587: Binary tree root-to-leaf paths.
// Approach: DFS, accumulate current path, record at leaves. Time O(n), Space O(h).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	val         int
	left, right *Node
}

func rootToLeafPaths(root *Node) [][]int {
	var res [][]int
	var dfs func(node *Node, path []int)
	dfs = func(node *Node, path []int) {
		if node == nil {
			return
		}
		path = append(path, node.val)
		if node.left == nil && node.right == nil {
			cp := make([]int, len(path))
			copy(cp, path)
			res = append(res, cp)
		} else {
			dfs(node.left, path)
			dfs(node.right, path)
		}
	}
	dfs(root, nil)
	return res
}

func main() {
	root := &Node{val: 1,
		left:  &Node{val: 2},
		right: &Node{val: 3, left: &Node{val: 4}, right: &Node{val: 5}}}

	paths := rootToLeafPaths(root)
	parts := make([]string, len(paths))
	for i, p := range paths {
		nums := make([]string, len(p))
		for j, v := range p {
			nums[j] = fmt.Sprintf("%d", v)
		}
		parts[i] = "[" + strings.Join(nums, ", ") + "]"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
