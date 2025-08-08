// Day 80: Return a deepest node of a binary tree via DFS tracking depth.
// Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	Val         byte
	Left, Right *Node
}

func deepestNode(root *Node) byte {
	best := -1
	var res byte
	var dfs func(n *Node, depth int)
	dfs = func(n *Node, depth int) {
		if n == nil {
			return
		}
		if depth > best {
			best = depth
			res = n.Val
		}
		dfs(n.Left, depth+1)
		dfs(n.Right, depth+1)
	}
	dfs(root, 0)
	return res
}

func main() {
	a := &Node{Val: 'a',
		Left:  &Node{Val: 'b', Left: &Node{Val: 'd'}},
		Right: &Node{Val: 'c'}}
	fmt.Printf("%c\n", deepestNode(a)) // d
}
