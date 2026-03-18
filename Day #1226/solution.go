// Cousins in a binary tree: BFS level by level tracking parent; collect same-level
// nodes with a different parent than target. Time O(n), Space O(n).
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

type pair struct {
	node, parent *Node
}

func cousins(root *Node, target int) []int {
	res := []int{}
	if root == nil {
		return res
	}
	queue := []pair{{root, nil}}
	for len(queue) > 0 {
		var level []pair
		var targetParent *Node
		found := false
		var next []pair
		for _, cur := range queue {
			level = append(level, cur)
			if cur.node.Val == target {
				found = true
				targetParent = cur.parent
			}
			if cur.node.Left != nil {
				next = append(next, pair{cur.node.Left, cur.node})
			}
			if cur.node.Right != nil {
				next = append(next, pair{cur.node.Right, cur.node})
			}
		}
		if found {
			for _, p := range level {
				if p.node.Val != target && p.parent != targetParent {
					res = append(res, p.node.Val)
				}
			}
			return res
		}
		queue = next
	}
	return res
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2, Left: &Node{Val: 4}, Right: &Node{Val: 5}},
		Right: &Node{Val: 3, Right: &Node{Val: 6}},
	}
	res := cousins(root, 4)
	parts := make([]string, len(res))
	for i, v := range res {
		parts[i] = strconv.Itoa(v)
	}
	fmt.Println("[" + strings.Join(parts, ", ") + "]")
}
