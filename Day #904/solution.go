// DFS tracking depth; record the node value seen at maximum depth. Time O(n), Space O(h).
package main

import "fmt"

type Node struct {
	val         string
	left, right *Node
}

func deepestNode(root *Node) string {
	maxDepth := -1
	deepest := ""
	var dfs func(node *Node, depth int)
	dfs = func(node *Node, depth int) {
		if node == nil {
			return
		}
		if depth > maxDepth {
			maxDepth = depth
			deepest = node.val
		}
		dfs(node.left, depth+1)
		dfs(node.right, depth+1)
	}
	dfs(root, 0)
	return deepest
}

func main() {
	a := &Node{val: "a"}
	a.left = &Node{val: "b"}
	a.right = &Node{val: "c"}
	a.left.left = &Node{val: "d"}
	fmt.Println(deepestNode(a))
}
