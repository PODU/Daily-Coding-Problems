// Day 110: Root-to-leaf paths via DFS backtracking. O(n) nodes, O(h) stack.
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

func dfs(n *Node, path []int, res *[][]int) {
	if n == nil {
		return
	}
	path = append(path, n.Val)
	if n.Left == nil && n.Right == nil {
		cp := make([]int, len(path))
		copy(cp, path)
		*res = append(*res, cp)
	} else {
		dfs(n.Left, path, res)
		dfs(n.Right, path, res)
	}
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2},
		Right: &Node{Val: 3, Left: &Node{Val: 4}, Right: &Node{Val: 5}}}
	var res [][]int
	dfs(root, nil, &res)
	parts := []string{}
	for _, p := range res {
		nums := []string{}
		for _, v := range p {
			nums = append(nums, strconv.Itoa(v))
		}
		parts = append(parts, "["+strings.Join(nums, ", ")+"]")
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
