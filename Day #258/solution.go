// Day 258: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing the output order on alternate levels.
// Time: O(n), Space: O(n).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type TreeNode struct {
	val   int
	left  *TreeNode
	right *TreeNode
}

func boustrophedon(root *TreeNode) []int {
	out := []int{}
	if root == nil {
		return out
	}
	queue := []*TreeNode{root}
	leftToRight := true
	for len(queue) > 0 {
		sz := len(queue)
		level := make([]int, sz)
		next := []*TreeNode{}
		for i := 0; i < sz; i++ {
			node := queue[i]
			idx := i
			if !leftToRight {
				idx = sz - 1 - i
			}
			level[idx] = node.val
			if node.left != nil {
				next = append(next, node.left)
			}
			if node.right != nil {
				next = append(next, node.right)
			}
		}
		out = append(out, level...)
		queue = next
		leftToRight = !leftToRight
	}
	return out
}

func main() {
	root := &TreeNode{1,
		&TreeNode{2, &TreeNode{4, nil, nil}, &TreeNode{5, nil, nil}},
		&TreeNode{3, &TreeNode{6, nil, nil}, &TreeNode{7, nil, nil}}}
	res := boustrophedon(root)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]") // [1, 3, 2, 4, 5, 6, 7]
}
