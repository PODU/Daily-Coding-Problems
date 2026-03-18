// Count unival subtrees via post-order DFS; node is unival if both children unival and values match.
// Time: O(n), Space: O(h) recursion.
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

// returns true if subtree is unival; increments *count.
func dfs(n *Node, count *int) bool {
	if n == nil {
		return true
	}
	l := dfs(n.left, count)
	r := dfs(n.right, count)
	if !l || !r {
		return false
	}
	if n.left != nil && n.left.val != n.val {
		return false
	}
	if n.right != nil && n.right.val != n.val {
		return false
	}
	*count++
	return true
}

func main() {
	root := &Node{val: 0,
		left: &Node{val: 1},
		right: &Node{val: 0,
			left:  &Node{val: 1, left: &Node{val: 1}, right: &Node{val: 1}},
			right: &Node{val: 0}}}
	count := 0
	dfs(root, &count)
	fmt.Println(count)
}
