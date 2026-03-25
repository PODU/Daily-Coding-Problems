// Day 1263: All root-to-leaf paths in a binary tree.
// DFS carrying the current path. O(n) nodes visited, O(h) recursion + output size.
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func rootToLeaf(root *Node) [][]int {
	var res [][]int
	var dfs func(n *Node, path []int)
	dfs = func(n *Node, path []int) {
		if n == nil {
			return
		}
		path = append(path, n.Val)
		if n.Left == nil && n.Right == nil {
			cp := make([]int, len(path))
			copy(cp, path)
			res = append(res, cp)
		} else {
			dfs(n.Left, path)
			dfs(n.Right, path)
		}
	}
	dfs(root, nil)
	return res
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2},
		Right: &Node{Val: 3, Left: &Node{Val: 4}, Right: &Node{Val: 5}},
	}
	fmt.Println(rootToLeaf(root))
}
