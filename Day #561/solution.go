// Sorted array -> height-balanced BST: recurse on mid=(lo+hi)/2 as root.
// Time: O(N), Space: O(N) for nodes + O(log N) recursion.
package main

import (
	"fmt"
	"strings"
)

type TreeNode struct {
	Val         int
	Left, Right *TreeNode
}

func build(a []int, lo, hi int) *TreeNode {
	if lo > hi {
		return nil
	}
	mid := (lo + hi) / 2
	root := &TreeNode{Val: a[mid]}
	root.Left = build(a, lo, mid-1)
	root.Right = build(a, mid+1, hi)
	return root
}

func main() {
	a := []int{1, 2, 3, 4, 5, 6, 7}
	root := build(a, 0, len(a)-1)
	var out []string
	queue := []*TreeNode{}
	if root != nil {
		queue = append(queue, root)
	}
	for len(queue) > 0 {
		n := queue[0]
		queue = queue[1:]
		out = append(out, fmt.Sprintf("%d", n.Val))
		if n.Left != nil {
			queue = append(queue, n.Left)
		}
		if n.Right != nil {
			queue = append(queue, n.Right)
		}
	}
	fmt.Println(strings.Join(out, " "))
}
