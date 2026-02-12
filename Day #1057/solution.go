// Generate all distinct BSTs with values 1..N: recursively pick each value as
// root, combine all left/right subtree shapes. Count is Catalan(N).
// Time/Space O(Catalan(N) * N).
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

func build(lo, hi int) []*TreeNode {
	if lo > hi {
		return []*TreeNode{nil}
	}
	var res []*TreeNode
	for root := lo; root <= hi; root++ {
		lefts := build(lo, root-1)
		rights := build(root+1, hi)
		for _, l := range lefts {
			for _, r := range rights {
				node := &TreeNode{val: root, left: l, right: r}
				res = append(res, node)
			}
		}
	}
	return res
}

func preorder(node *TreeNode, out *strings.Builder) {
	if node == nil {
		out.WriteString("#")
		return
	}
	out.WriteString(strconv.Itoa(node.val))
	out.WriteString(" ")
	preorder(node.left, out)
	out.WriteString(" ")
	preorder(node.right, out)
}

func main() {
	N := 3
	trees := build(1, N)
	fmt.Println(len(trees)) // 5 for N=3
	for _, t := range trees {
		var sb strings.Builder
		preorder(t, &sb)
		fmt.Println(sb.String())
	}
}
