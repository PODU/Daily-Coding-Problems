// Root-to-leaf paths via DFS, appending to the current path and recording it at each leaf.
// Time O(n) nodes (O(n*h) including path copies), Space O(h) recursion.
package main

import "fmt"

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

func rootToLeafPaths(root *Node) [][]int {
	var res [][]int
	dfs(root, []int{}, &res)
	return res
}

func main() {
	root := &Node{Val: 1,
		Left:  &Node{Val: 2},
		Right: &Node{Val: 3, Left: &Node{Val: 4}, Right: &Node{Val: 5}}}
	fmt.Println(rootToLeafPaths(root)) // [[1 2] [1 3 4] [1 3 5]]
}
