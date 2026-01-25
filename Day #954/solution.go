// Day 954: count unival subtrees (all nodes in subtree share one value).
// Post-order DFS, returning whether the subtree is unival. Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func countUnival(root *Node) int {
	count := 0
	var dfs func(*Node) bool
	dfs = func(node *Node) bool {
		if node == nil {
			return true
		}
		l := dfs(node.left)
		r := dfs(node.right)
		if !l || !r {
			return false
		}
		if node.left != nil && node.left.val != node.val {
			return false
		}
		if node.right != nil && node.right.val != node.val {
			return false
		}
		count++
		return true
	}
	dfs(root)
	return count
}

func main() {
	root := &Node{val: 0,
		left: &Node{val: 1},
		right: &Node{val: 0,
			left:  &Node{val: 1, left: &Node{val: 1}, right: &Node{val: 1}},
			right: &Node{val: 0}}}
	fmt.Println(countUnival(root)) // 5
}
