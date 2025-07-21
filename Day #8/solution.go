// Count unival subtrees: postorder; a subtree is unival if both children are
// unival and match the node's value. Time: O(n), Space: O(h).
package main

import "fmt"

type Node struct {
	Val         int
	Left, Right *Node
}

func countUnival(root *Node) int {
	count := 0
	var isUnival func(n *Node) bool
	isUnival = func(n *Node) bool {
		if n == nil {
			return true
		}
		l := isUnival(n.Left)
		r := isUnival(n.Right)
		if !l || !r {
			return false
		}
		if n.Left != nil && n.Left.Val != n.Val {
			return false
		}
		if n.Right != nil && n.Right.Val != n.Val {
			return false
		}
		count++
		return true
	}
	isUnival(root)
	return count
}

func main() {
	root := &Node{0,
		&Node{1, nil, nil},
		&Node{0, &Node{1, &Node{1, nil, nil}, &Node{1, nil, nil}}, &Node{0, nil, nil}}}
	fmt.Println(countUnival(root))
}
