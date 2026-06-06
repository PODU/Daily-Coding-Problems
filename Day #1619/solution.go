// Subtree check via serialization with sentinels + substring search.
// Time: O(n+m), Space: O(n+m).
package main

import (
	"fmt"
	"strconv"
	"strings"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func serialize(node *TreeNode, sb *strings.Builder) {
	if node == nil {
		sb.WriteString(",#")
		return
	}
	sb.WriteString("," + strconv.Itoa(node.Val))
	serialize(node.Left, sb)
	serialize(node.Right, sb)
}

func isSubtree(s, t *TreeNode) bool {
	var ss, ts strings.Builder
	serialize(s, &ss)
	serialize(t, &ts)
	return strings.Contains(ss.String(), ts.String())
}

func main() {
	s := &TreeNode{Val: 3}
	s.Left = &TreeNode{Val: 4}
	s.Right = &TreeNode{Val: 5}
	s.Left.Left = &TreeNode{Val: 1}
	s.Left.Right = &TreeNode{Val: 2}

	t := &TreeNode{Val: 4}
	t.Left = &TreeNode{Val: 1}
	t.Right = &TreeNode{Val: 2}

	if isSubtree(s, t) {
		fmt.Println("true")
	} else {
		fmt.Println("false")
	}
}
