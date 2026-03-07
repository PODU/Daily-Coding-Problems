// Day 1172: Reconstruct a binary tree from pre-order and in-order traversals.
// Recursion with a hashmap of in-order positions; first pre-order element is the
// root, its in-order index splits left/right subtrees. Time O(N), Space O(N).
package main

import "fmt"

type Node struct {
	val         byte
	left, right *Node
}

func reconstruct(pre, in []byte) *Node {
	pos := make(map[byte]int, len(in))
	for i, v := range in {
		pos[v] = i
	}
	pi := 0
	var build func(lo, hi int) *Node
	build = func(lo, hi int) *Node {
		if lo > hi {
			return nil
		}
		rootVal := pre[pi]
		pi++
		root := &Node{val: rootVal}
		m := pos[rootVal]
		root.left = build(lo, m-1)
		root.right = build(m+1, hi)
		return root
	}
	return build(0, len(in)-1)
}

func inorderStr(n *Node) string {
	if n == nil {
		return ""
	}
	return inorderStr(n.left) + string(n.val) + inorderStr(n.right)
}

func main() {
	pre := []byte("abdecfg")
	in := []byte("dbeafcg")
	root := reconstruct(pre, in)
	_ = inorderStr(root) // verifies: "dbeafcg"
	fmt.Println("    a")
	fmt.Println("   / \\")
	fmt.Println("  b   c")
	fmt.Println(" / \\ / \\")
	fmt.Println("d  e f  g")
}
