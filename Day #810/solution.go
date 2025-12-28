// Day 810: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(N), Space O(N).
package main

import "fmt"

type Node struct {
	val         int
	left, right *Node
}

func boustrophedon(root *Node) []int {
	var out []int
	if root == nil {
		return out
	}
	queue := []*Node{root}
	ltr := true
	for len(queue) > 0 {
		var next []*Node
		level := make([]int, 0, len(queue))
		for _, n := range queue {
			level = append(level, n.val)
			if n.left != nil {
				next = append(next, n.left)
			}
			if n.right != nil {
				next = append(next, n.right)
			}
		}
		if !ltr {
			for i, j := 0, len(level)-1; i < j; i, j = i+1, j-1 {
				level[i], level[j] = level[j], level[i]
			}
		}
		out = append(out, level...)
		ltr = !ltr
		queue = next
	}
	return out
}

func main() {
	root := &Node{val: 1}
	root.left = &Node{val: 2}
	root.right = &Node{val: 3}
	root.left.left = &Node{val: 4}
	root.left.right = &Node{val: 5}
	root.right.left = &Node{val: 6}
	root.right.right = &Node{val: 7}
	fmt.Println(boustrophedon(root)) // [1 3 2 4 5 6 7]
}
