// Day 644: Count unival subtrees (all nodes share one value).
// Approach: post-order DFS; a node is unival iff both children are unival and
// their values match the node's. Count as we recurse.
// Time: O(n), Space: O(h).
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
		left := dfs(node.left)
		right := dfs(node.right)
		if !left || !right {
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
