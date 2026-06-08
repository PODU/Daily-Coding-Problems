// Root-to-leaf paths via DFS, carrying current path; record at leaves. Time O(n), Space O(h).
package main

import (
	"fmt"
	"strings"
)

type Node struct {
	Val         int
	Left, Right *Node
}

func dfs(node *Node, path []int, res *[][]int) {
	if node == nil {
		return
	}
	path = append(path, node.Val)
	if node.Left == nil && node.Right == nil {
		cp := make([]int, len(path))
		copy(cp, path)
		*res = append(*res, cp)
	} else {
		dfs(node.Left, path, res)
		dfs(node.Right, path, res)
	}
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2},
		Right: &Node{Val: 3, Left: &Node{Val: 4}, Right: &Node{Val: 5}}}

	var res [][]int
	dfs(root, []int{}, &res)

	parts := make([]string, len(res))
	for i, p := range res {
		nums := make([]string, len(p))
		for j, v := range p {
			nums[j] = fmt.Sprintf("%d", v)
		}
		parts[i] = "[" + strings.Join(nums, ", ") + "]"
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
